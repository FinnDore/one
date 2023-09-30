#![no_std]
#![no_main]
#![feature(type_alias_impl_trait)]
#![feature(alloc_error_handler)]
mod animations;
mod ws2812;

use core::cell::{Ref, RefCell};
use core::mem::size_of;

use alloc::boxed::Box;
use animations::{NextFrame, StaticColorAnimation};
use core::mem::MaybeUninit;
use defmt::*;
use embassy_executor::{Executor, Spawner};
use embassy_rp::bind_interrupts;
use embassy_rp::gpio::{Input, Pull};
use embassy_rp::multicore::{spawn_core1, Stack};
use embassy_rp::peripherals::{DMA_CH0, PIN_15, PIN_19, PIO0};
use embassy_rp::pio::{InterruptHandler, Pio, StateMachine};
use embassy_sync::blocking_mutex::raw::{NoopRawMutex, RawMutex, ThreadModeRawMutex};
use embassy_sync::blocking_mutex::{CriticalSectionMutex, Mutex};
use embassy_time::{Duration, Timer};
use heapless::{arc_pool, Arc};
use smart_leds::colors::{
    AQUA, HOT_PINK, LAVENDER_BLUSH, ORANGE, ORANGE_RED, PURPLE, VIOLET, WHITE, YELLOW,
};
use smart_leds::RGB8;
use static_cell::StaticCell;

extern crate alloc;

use crate::animations::currrent_frame;
use crate::ws2812::Ws2812;
use {defmt_rtt as _, panic_probe as _};

use alloc_cortex_m::CortexMHeap;
#[global_allocator] // 👈
static ALLOCATOR: CortexMHeap = CortexMHeap::empty();
const COLORS: [RGB8; 10] = [
    HOT_PINK,
    PURPLE,
    YELLOW,
    AQUA,
    VIOLET,
    ORANGE_RED,
    WHITE,
    LAVENDER_BLUSH,
    YELLOW,
    ORANGE,
];
const NUM_LEDS: usize = 1;

static mut CORE1_STACK: Stack<4096> = Stack::new();
static EXECUTOR1: StaticCell<Executor> = StaticCell::new();
static EXECUTOR0: StaticCell<Executor> = StaticCell::new();

arc_pool!(P: Mutex<ThreadModeRawMutex,  StaticColorAnimation>>);

#[embassy_executor::task]
pub async fn color_task(pio0: PIO0, data_pin: PIN_19, dma: DMA_CH0, current_animation: Arc<P>) {
    debug!("Core 2 started");
    let Pio {
        mut common, sm0, ..
    } = Pio::new(pio0, Irqs);
    let mut ws2812 = Ws2812::new(&mut common, sm0, dma, data_pin, [WHITE; NUM_LEDS]);

    loop {
        current_animation
            .lock(|animation| ws2812.write_all_colors(animation.current_frame().clone()));

        debug!("Button pressed color changed");
        Timer::after(Duration::from_millis(50)).await;
    }
}

#[embassy_executor::main]
async fn main(_spawner: Spawner) {
    debug!("Program started");

    let p = embassy_rp::init(Default::default());
    let mut s = StaticColorAnimation::new(COLORS.to_vec());
    let current_animation_arc: Arc<P> = P::alloc(Mutex::new(s)).ok().expect("alloc");

    let current_animation_for_task = current_animation_arc.clone();
    let current_animation = current_animation_arc.clone().as_ref();

    spawn_core1(p.CORE1, unsafe { &mut CORE1_STACK }, || {
        let executor1 = EXECUTOR1.init(Executor::new());
        executor1.run(|spawner| {
            unwrap!(spawner.spawn(color_task(
                p.PIO0,
                p.PIN_19,
                p.DMA_CH0,
                current_animation_for_task
            )))
        });
    });

    let mut button = Input::new(p.PIN_15, Pull::Up);
    loop {
        wait_for_button_press(&mut button).await;
        let c = current_animation.get_mut();
        c.next_frame();
        drop(c);
    }
}

async fn wait_for_button_press(button: &mut Input<'_, PIN_15>) {
    button.wait_for_low().await;
    button.wait_for_high().await;
}

bind_interrupts!(struct Irqs {
    PIO0_IRQ_0 => InterruptHandler<PIO0>;
});

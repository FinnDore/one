#![no_std]
#![no_main]
#![feature(type_alias_impl_trait)]

mod animations;
mod ws2812;

use core::cell::RefCell;

use animations::{NextFrame, StaticColorAnimation};
use defmt::*;
use embassy_executor::{Executor, Spawner};
use embassy_rp::bind_interrupts;
use embassy_rp::gpio::{Input, Pull};
use embassy_rp::multicore::{pause_core1, resume_core1, spawn_core1, Stack};
use embassy_rp::peripherals::{CORE1, DMA_CH0, PIN_15, PIN_19, PIO0};
use embassy_rp::pio::{InterruptHandler, Pio};
use embassy_sync::blocking_mutex::raw::ThreadModeRawMutex;
use embassy_sync::blocking_mutex::Mutex;
use embassy_time::{Duration, Timer};
use futures::future::Lazy;
use smart_leds::colors::{
    AQUA, HOT_PINK, LAVENDER_BLUSH, ORANGE, ORANGE_RED, PURPLE, VIOLET, WHITE, YELLOW,
};
use smart_leds::{RGB, RGB8};
use static_cell::StaticCell;

// extern crate alloc;

use crate::animations::CurrentFrame;
use crate::ws2812::Ws2812;
use {defmt_rtt as _, panic_probe as _};

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

static STATE: Mutex<ThreadModeRawMutex, RefCell<StaticColorAnimation>> =
    Mutex::new(RefCell::new(StaticColorAnimation::new(COLORS)));

#[embassy_executor::task]
pub async fn color_task(pio0: PIO0, data_pin: PIN_19, dma: DMA_CH0) {
    debug!("Core 2 started");
    let Pio {
        mut common, sm0, ..
    } = Pio::new(pio0, Irqs);
    let mut ws2812 = Ws2812::new(&mut common, sm0, dma, data_pin, [WHITE; NUM_LEDS]);

    let mut should_sleep: bool;
    loop {
        should_sleep = false;
        let current_state = STATE.lock(|cur| {
            let current_animation = cur.borrow();
            if current_animation.is_static {
                should_sleep = true;
            }
            return current_animation.current_frame().clone();
        });

        ws2812.write_all_colors(current_state).await;
        Timer::after(Duration::from_millis(10)).await;
        if should_sleep {
            debug!("sleeeping");
            pause_core1();
        }
    }
}

#[embassy_executor::main]
async fn main(_spawner: Spawner) {
    debug!("Program started");

    let p = embassy_rp::init(Default::default());

    spawn_core1(p.CORE1, unsafe { &mut CORE1_STACK }, || {
        let executor1 = EXECUTOR1.init(Executor::new());
        executor1.run(|spawner| unwrap!(spawner.spawn(color_task(p.PIO0, p.PIN_19, p.DMA_CH0))));
    });

    let mut button = Input::new(p.PIN_15, Pull::Up);
    let mut is_static = false;
    println!("aaaa");
    loop {
        let should_resume = is_static.clone();
        is_static = STATE.lock(|cur| {
            let mut current_animation = cur.borrow_mut();
            current_animation.next_frame();
            return current_animation.is_static.clone();
        });

        debug!("is_static: {}", is_static);
        if should_resume {
            debug!("try resume");
            resume_core1();
            debug!("resume");
        }
        wait_for_button_press(&mut button).await;
    }
}

async fn wait_for_button_press(button: &mut Input<'_, PIN_15>) {
    button.wait_for_low().await;
    button.wait_for_high().await;
}

bind_interrupts!(struct Irqs {
    PIO0_IRQ_0 => InterruptHandler<PIO0>;
});

#![no_std]
#![no_main]
#![feature(type_alias_impl_trait)]

mod animations;
mod ws2812;

use core::cell::RefCell;

use animations::AnimationSet;

use defmt::*;
use embassy_executor::{Executor, InterruptExecutor, Spawner};
use embassy_rp::gpio::{Input, Pull};
use embassy_rp::interrupt::{InterruptExt, Priority};
use embassy_rp::multicore::{spawn_core1, Stack};
use embassy_rp::peripherals::{DMA_CH0, PIN_15, PIO0};
use embassy_rp::pio::{InterruptHandler, Pio};
use embassy_rp::{bind_interrupts, interrupt};
use embassy_sync::blocking_mutex::raw::ThreadModeRawMutex;
use embassy_sync::blocking_mutex::Mutex;
use embassy_time::{Duration, Timer};
use smart_leds::RGBW;
use static_cell::StaticCell;

// extern crate alloc;

use crate::ws2812::Ws2812;
use {defmt_rtt as _, panic_probe as _};

const NUM_LEDS: usize = 64;

static mut CORE1_STACK: Stack<4096> = Stack::new();
static EXECUTOR1: StaticCell<Executor> = StaticCell::new();

static EXECUTOR0: InterruptExecutor = InterruptExecutor::new();

static EXECUTOR2: StaticCell<Executor> = StaticCell::new();

static STATE: Mutex<ThreadModeRawMutex, RefCell<AnimationSet>> =
    Mutex::new(RefCell::new(AnimationSet::new()));

#[interrupt]
unsafe fn SWI_IRQ_1() {
    EXECUTOR0.on_interrupt()
}

#[embassy_executor::task]
pub async fn color_task(pio0: PIO0, data_pin: embassy_rp::peripherals::PIN_16, dma: DMA_CH0) {
    debug!("Color task started");
    let Pio {
        mut common, sm0, ..
    } = Pio::new(pio0, Irqs);
    let mut ws2812 = Ws2812::new(
        &mut common,
        sm0,
        dma,
        data_pin,
        [RGBW::new_alpha(255, 255, 255, smart_leds::White(0)); NUM_LEDS],
    );

    loop {
        let current_state = STATE.lock(|cur| {
            let mut animation_set = cur.borrow_mut();
            let current_animation = animation_set.current_animation();

            if current_animation.is_static() {
                return *current_animation.current_frame();
            } else {
                return *current_animation.next_frame();
            }
        });

        ws2812.write_all_colors(current_state).await;
        Timer::after(Duration::from_millis(50)).await;
    }
}

#[embassy_executor::task]
pub async fn wifi_task() {
    debug!("wifi task");
    loop {
        Timer::after(Duration::from_secs(2)).await;
    }
}

#[embassy_executor::task]
pub async fn button_task(button_pin: PIN_15) {
    debug!("Button task started");
    let mut button = Input::new(button_pin, Pull::Up);
    loop {
        wait_for_button_press(&mut button).await;
        STATE.lock(|cur| cur.borrow_mut().next_animation());
    }
}

#[embassy_executor::main]
async fn main(_main_spawner: Spawner) {
    debug!("Program started");
    let p = embassy_rp::init(Default::default());

    spawn_core1(p.CORE1, unsafe { &mut CORE1_STACK }, || {
        let executor1 = EXECUTOR1.init(Executor::new());
        executor1.run(|spawner| unwrap!(spawner.spawn(color_task(p.PIO0, p.PIN_16, p.DMA_CH0))));
    });

    interrupt::SWI_IRQ_1.set_priority(Priority::P0);
    let s = EXECUTOR0.start(interrupt::SWI_IRQ_1);
    s.spawn(wifi_task()).expect("Wifi task failed to spawn");

    let executor = EXECUTOR2.init(Executor::new());
    executor.run(|spawner| {
        spawner
            .spawn(button_task(p.PIN_15))
            .expect("Button task failed to spawn");
    });
}

async fn wait_for_button_press(button: &mut Input<'_, PIN_15>) {
    button.wait_for_low().await;
    button.wait_for_high().await;
}

bind_interrupts!(struct Irqs {
    PIO0_IRQ_0 => InterruptHandler<PIO0>;
});

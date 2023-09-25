#![no_std]
#![no_main]
#![feature(type_alias_impl_trait)]

mod ws2812;

use defmt::*;
use embassy_executor::{Executor, Spawner};
use embassy_rp::bind_interrupts;
use embassy_rp::gpio::{Input, Pull};
use embassy_rp::multicore::{spawn_core1, Stack};
use embassy_rp::peripherals::{DMA_CH0, PIN_15, PIN_19, PIO0};
use embassy_rp::pio::{InterruptHandler, Pio};
use embassy_time::{Duration, Timer};
use smart_leds::colors::{
    AQUA, HOT_PINK, LAVENDER_BLUSH, ORANGE, ORANGE_RED, PURPLE, VIOLET, WHITE, YELLOW,
};
use smart_leds::RGB8;
use static_cell::StaticCell;

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
static EXECUTOR0: StaticCell<Executor> = StaticCell::new();

#[embassy_executor::main]
async fn main(_spawner: Spawner) {
    debug!("Program started");
    let p = embassy_rp::init(Default::default());

    spawn_core1(p.CORE1, unsafe { &mut CORE1_STACK }, move || {
        let executor1 = EXECUTOR0.init(Executor::new());
        executor1.run(move |spawner| {
            unwrap!(spawner.spawn(color_task(p.PIO0, p.PIN_19, p.PIN_15, p.DMA_CH0)))
        });
    });
}

#[embassy_executor::task]
async fn color_task(pio0: PIO0, data_pin: PIN_19, button_pin: PIN_15, dma: DMA_CH0) {
    debug!("Core 2 started");
    let Pio {
        mut common, sm0, ..
    } = Pio::new(pio0, Irqs);

    let mut ws2812 = Ws2812::new(&mut common, sm0, dma, data_pin, [WHITE; NUM_LEDS]);
    let mut button = Input::new(button_pin, Pull::Up);

    loop {
        for color in COLORS {
            ws2812.write_all_colors(color).await;
            wait_for_button_press(&mut button).await;
            debug!("Button pressed color changed");
            Timer::after(Duration::from_millis(50)).await;
        }
    }
}

async fn wait_for_button_press(button: &mut Input<'_, PIN_15>) {
    button.wait_for_low().await;
    button.wait_for_high().await;
}

bind_interrupts!(struct Irqs {
    PIO0_IRQ_0 => InterruptHandler<PIO0>;
});

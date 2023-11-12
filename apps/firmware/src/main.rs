#![no_std]
#![no_main]
#![feature(type_alias_impl_trait)]

mod animations;
mod color;
mod shared;
mod tasks;
mod utils;
mod ws2812;

use tasks::tcp::tcp_task;

use defmt::*;
use embassy_executor::{Executor, InterruptExecutor, Spawner};

use embassy_rp::interrupt::{InterruptExt, Priority};
use embassy_rp::multicore::{spawn_core1, Stack};
use embassy_rp::peripherals::PIO0;
use embassy_rp::pio::{InterruptHandler, Pio};
use embassy_rp::{bind_interrupts, interrupt};

use smart_leds::RGBW;
use static_cell::StaticCell;

use crate::color::Color;
use crate::shared::NUM_LEDS;

use crate::tasks::button::button_task;
use crate::tasks::color::color_task;
use crate::tasks::tcp::TcpTaskOpts;
use crate::ws2812::Ws2812;
use {defmt_rtt as _, panic_probe as _};

static mut CORE1_STACK: Stack<8192> = Stack::new();
static EXECUTOR1: StaticCell<Executor> = StaticCell::new();

static EXECUTOR0: InterruptExecutor = InterruptExecutor::new();

#[interrupt]
unsafe fn SWI_IRQ_1() {
    EXECUTOR0.on_interrupt()
}

#[embassy_executor::main]
async fn main(main_spawner: Spawner) {
    debug!("Program started");
    let p = embassy_rp::init(Default::default());

    let mut pio = Pio::new(p.PIO0, Irqs);

    interrupt::SWI_IRQ_1.set_priority(Priority::P0);
    let s = EXECUTOR0.start(interrupt::SWI_IRQ_1);
    s.spawn(button_task(p.PIN_15))
        .expect("Button task failed to spawn");

    let ws2812 = Ws2812::new(
        &mut pio.common,
        pio.sm1,
        p.DMA_CH1,
        p.PIN_16,
        [Color::default(); NUM_LEDS],
    );

    spawn_core1(p.CORE1, unsafe { &mut CORE1_STACK }, || {
        let executor1 = EXECUTOR1.init(Executor::new());
        executor1.run(|spawner| {
            spawner
                .spawn(color_task(ws2812))
                .expect("Color task failed to spawn")
        });
    });

    tcp_task(
        main_spawner,
        TcpTaskOpts {
            pin_23: p.PIN_23,
            pin_24: p.PIN_24,
            pin_25: p.PIN_25,
            pin_29: p.PIN_29,
            dma_ch0: p.DMA_CH0,
            sm0: pio.sm0,
            irq0: pio.irq0,
        },
        pio.common,
    )
    .await;
}

bind_interrupts!(struct Irqs {
    PIO0_IRQ_0 => InterruptHandler<PIO0>;
});

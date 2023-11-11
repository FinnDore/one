use cyw43_pio::PioSpi;
use defmt::*;
use embassy_executor::{Executor, InterruptExecutor, Spawner};
use embassy_net::tcp::TcpSocket;
use embassy_net::{Config, StackResources};
use embassy_rp::gpio::{Input, Level, Output, Pull};
use embassy_rp::interrupt::{InterruptExt, Priority};
use embassy_rp::multicore::{spawn_core1, Stack};
use embassy_rp::peripherals::{DMA_CH0, PIN_15, PIN_23, PIN_24, PIN_25, PIN_29, PIO0};
use embassy_rp::pio::{Common, InterruptHandler, Irq, Pio, StateMachine};
use embassy_rp::{bind_interrupts, interrupt};
use embassy_sync::blocking_mutex::raw::CriticalSectionRawMutex;
use embassy_sync::blocking_mutex::Mutex;
use embassy_time::{Duration, Timer};

use smart_leds::RGBW;
use static_cell::StaticCell;

use crate::ws2812::Ws2812;
use crate::{NUM_LEDS, STATE};

#[embassy_executor::task]
pub async fn color_task(mut ws2812: Ws2812<'static, PIO0, 1, NUM_LEDS>) {
    debug!("Color task started");

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

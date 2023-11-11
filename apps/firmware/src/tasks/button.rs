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

use crate::STATE;

#[embassy_executor::task]
pub async fn button_task(button_pin: PIN_15) {
    debug!("Button task started");
    let mut button = Input::new(button_pin, Pull::Up);
    loop {
        wait_for_button_press(&mut button).await;
        STATE.lock(|cur| cur.borrow_mut().next_animation());
    }
}

async fn wait_for_button_press(button: &mut Input<'_, PIN_15>) {
    button.wait_for_low().await;
    button.wait_for_high().await;
}

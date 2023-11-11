use defmt::*;

use embassy_rp::gpio::{Input, Pull};

use embassy_rp::peripherals::PIN_15;

use crate::shared::STATE;

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

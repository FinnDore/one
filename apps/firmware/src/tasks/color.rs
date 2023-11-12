use defmt::*;

use embassy_rp::peripherals::PIO0;

use embassy_time::{Duration, Timer};

use crate::shared::{NUM_LEDS, STATE};
use crate::ws2812::Ws2812;

#[embassy_executor::task]
pub async fn color_task(mut ws2812: Ws2812<'static, PIO0, 1, NUM_LEDS>) {
    debug!("Color task started");

    loop {
        let current_state = STATE.lock(|cur| {
            let mut animation_set = cur.borrow_mut();
            let current_animation = animation_set.current_animation();

            if current_animation.is_static() {
                *current_animation.current_frame()
            } else {
                *current_animation.next_frame()
            }
        });

        ws2812.write_all_colors(current_state).await;
        Timer::after(Duration::from_millis(50)).await;
    }
}

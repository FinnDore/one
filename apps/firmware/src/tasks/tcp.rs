use core::str::from_utf8;

use embassy_executor::Spawner;

use cyw43_pio::PioSpi;
use defmt::*;
use embassy_net::tcp::TcpSocket;
use embassy_net::{Config, StackResources};
use embassy_rp::gpio::{Level, Output};

use embassy_rp::peripherals::{DMA_CH0, PIN_23, PIN_24, PIN_25, PIN_29, PIO0};
use embassy_rp::pio::{Common, Irq, StateMachine};

use embassy_time::{Duration, Timer};

use static_cell::make_static;

use crate::{utils::hex::hex_to_rgbw, STATE};
use {defmt_rtt as _, panic_probe as _};

pub struct TcpTaskOpts {
    pub pin_23: PIN_23,
    pub pin_24: PIN_24,
    pub pin_25: PIN_25,
    pub pin_29: PIN_29,
    pub dma_ch0: DMA_CH0,
    pub sm0: StateMachine<'static, PIO0, 0>,
    pub irq0: Irq<'static, PIO0, 0>,
}

#[embassy_executor::task]
async fn wifi_task(
    runner: cyw43::Runner<
        'static,
        Output<'static, PIN_23>,
        PioSpi<'static, PIN_25, PIO0, 0, DMA_CH0>,
    >,
) -> ! {
    runner.run().await
}

#[embassy_executor::task]
async fn net_task(stack: &'static embassy_net::Stack<cyw43::NetDriver<'static>>) -> ! {
    stack.run().await
}

pub async fn tcp_task(spawner: Spawner, opts: TcpTaskOpts, mut common: Common<'static, PIO0>) -> ! {
    info!("tcp task started");

    // WIFI
    let firmware = include_bytes!("../../cyw43-firmware/43439A0.bin");
    let clm = include_bytes!("../../cyw43-firmware/43439A0_clm.bin");

    let pwr = Output::new(opts.pin_23, Level::Low);
    let cs = Output::new(opts.pin_25, Level::High);
    let spi = PioSpi::new(
        &mut common,
        opts.sm0,
        opts.irq0,
        cs,
        opts.pin_24,
        opts.pin_29,
        opts.dma_ch0,
    );

    let state = make_static!(cyw43::State::new());
    let (net_device, mut control, runner) = cyw43::new(state, pwr, spi, firmware).await;
    spawner
        .spawn(wifi_task(runner))
        .expect("Wifi task failed to spawn");

    control.init(clm).await;
    control
        .set_power_management(cyw43::PowerManagementMode::PowerSave)
        .await;

    let config = Config::dhcpv4(Default::default());

    // Generate random seed
    let seed = 0x0123_4567_89ab_cdef; // chosen by fair dice roll. guarenteed to be random.
                                      // Init network stack
    let stack = &*make_static!(embassy_net::Stack::new(
        net_device,
        config,
        make_static!(StackResources::<2>::new()),
        seed
    ));

    spawner
        .spawn(net_task(stack))
        .expect("net task failed to spawn");
    loop {
        match control.join_wpa2("2.5G", "dynamicrabbit205").await {
            Ok(_) => break,
            Err(err) => {
                debug!("join failed with status={}", err.status);
            }
        }
    }

    // Wait for DHCP, not necessary when using static IP
    debug!("waiting for DHCP...");
    while !stack.is_config_up() {
        Timer::after(Duration::from_millis(100)).await;
    }
    debug!("DHCP is now up!");

    let mut rx_buffer = [0; 4096];
    let mut tx_buffer = [0; 4096];
    let mut buf = [0; 4096];

    loop {
        let mut socket = TcpSocket::new(stack, &mut rx_buffer, &mut tx_buffer);
        socket.set_timeout(Some(Duration::from_secs(600)));

        control.gpio_set(0, false).await;
        info!("Listening on TCP:1234...");
        if let Err(e) = socket.accept(1234).await {
            warn!("accept error: {:?}", e);
            continue;
        }

        info!("Received connection from {:?}", socket.remote_endpoint());
        control.gpio_set(0, true).await;

        loop {
            let n = match socket.read(&mut buf).await {
                Ok(0) => {
                    warn!("read EOF");
                    break;
                }
                Ok(n) => n,
                Err(e) => {
                    warn!("read error: {:?}", e);
                    break;
                }
            };

            match from_utf8(&buf[..n]) {
                Ok(res) => {
                    info!("received: {}", res);
                    handle(res);
                }
                Err(_) => warn!("invalid UTF-8"),
            }
        }
    }
}

fn handle(req: &str) {
    let parse_result = hex_to_rgbw(req);

    if parse_result.is_err() {
        warn!("invalid hex");
        return;
    }

    let (_, color) = parse_result.unwrap();
    STATE.lock(|cur| {
        let mut animation_set = cur.borrow_mut();

        animation_set.set_color(color);
    });
    info!("color changed to {}{}{}", color.r, color.g, color.b);
}

#![no_std]
#![no_main]
#![feature(type_alias_impl_trait)]

mod animations;
mod ws2812;

use core::cell::RefCell;
use core::str::from_utf8;

use animations::AnimationSet;

use cyw43_pio::PioSpi;
use defmt::*;
use embassy_executor::{Executor, InterruptExecutor, Spawner};
use embassy_net::tcp::TcpSocket;
use embassy_net::{Config, IpAddress, Ipv4Address, Ipv4Cidr, StackResources};
use embassy_rp::gpio::{AnyPin, Input, Level, Output, Pull};
use embassy_rp::interrupt::{InterruptExt, Priority};
use embassy_rp::multicore::{spawn_core1, Stack};
use embassy_rp::peripherals::{DMA_CH0, PIN_15, PIN_23, PIN_24, PIN_25, PIN_29, PIO0};
use embassy_rp::pio::{Common, InterruptHandler, Irq, Pio, StateMachine};
use embassy_rp::{bind_interrupts, interrupt};
use embassy_sync::blocking_mutex::raw::{CriticalSectionRawMutex, ThreadModeRawMutex};
use embassy_sync::blocking_mutex::Mutex;
use embassy_time::{Duration, Timer};
use embedded_io_async::Write;
use heapless::Vec;
use smart_leds::RGBW;
use static_cell::{make_static, StaticCell};

// extern crate alloc;

use crate::ws2812::Ws2812;
use {defmt_rtt as _, panic_probe as _};

const NUM_LEDS: usize = 64;

static mut CORE1_STACK: Stack<4096> = Stack::new();
static EXECUTOR1: StaticCell<Executor> = StaticCell::new();

static EXECUTOR0: InterruptExecutor = InterruptExecutor::new();
static EXECUTOR2: InterruptExecutor = InterruptExecutor::new();
static EXECUTOR3: StaticCell<Executor> = StaticCell::new();

static STATE: Mutex<CriticalSectionRawMutex, RefCell<AnimationSet>> =
    Mutex::new(RefCell::new(AnimationSet::new()));

#[interrupt]
unsafe fn SWI_IRQ_1() {
    EXECUTOR0.on_interrupt()
}

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

#[embassy_executor::task]
pub async fn wifi_task2() {
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

struct TcpTaskOpts {
    pin_23: PIN_23,
    pin_24: PIN_24,
    pin_25: PIN_25,
    pin_29: PIN_29,
    dma_ch0: DMA_CH0,
    sm0: StateMachine<'static, PIO0, 0>,
    irq0: Irq<'static, PIO0, 0>,
}

async fn tcp_task(spawner: Spawner, opts: TcpTaskOpts, mut common: Common<'static, PIO0>) -> ! {
    info!("tcp task started");

    // WIFI
    let firmware = include_bytes!("../cyw43-firmware/43439A0.bin");
    let clm = include_bytes!("../cyw43-firmware/43439A0_clm.bin");

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

        // control.gpio_set(0, false).await;
        info!("Listening on TCP:1234...");
        if let Err(e) = socket.accept(1234).await {
            warn!("accept error: {:?}", e);
            continue;
        }

        info!("Received connection from {:?}", socket.remote_endpoint());
        // control.gpio_set(0, true).await;

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

            if let Ok(hex) = from_utf8(&buf[..n]) {
                info!(" {}", hex);
                STATE.lock(|cur| {
                    let mut animation_set = cur.borrow_mut();
                    animation_set.setColor(hexToRgba(hex));
                });
                info!("color changed to {}", hex);
            } else {
                warn!("invalid UTF-8");
            }
        }
    }
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

    // interrupt::SWI_IRQ_2.set_priority(Priority::P0);
    let mut ws2812 = Ws2812::new(
        &mut pio.common,
        pio.sm1,
        p.DMA_CH1,
        p.PIN_16,
        [RGBW::new_alpha(255, 255, 255, smart_leds::White(0)); NUM_LEDS],
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

async fn wait_for_button_press(button: &mut Input<'_, PIN_15>) {
    button.wait_for_low().await;
    button.wait_for_high().await;
}

bind_interrupts!(struct Irqs {
    PIO0_IRQ_0 => InterruptHandler<PIO0>;
});

//! This example sends incomming udp packets to an endpoint depending on the state of input pin GP15. See button example first.
//! In order to connect to the wifi network please create the following two files in the `src` folder:
//! WIFI_SSID.txt and WIFI_PASSWORD.txt
//! The files above should contain the exact ssid and password to connect to the wifi network. No newline characters or quotes.
//!
//! This example is for a RP Pico W or PR Pico WH. It does not work with the RP Pico board (non-wifi).
//!
//! How to run with a standard usb cable (no debug probe):
//! The pico has a builtin bootloader that can be used as a replacement for a debug probe (like an ST link v2).
//! Start with the usb cable unplugged then, while holding down the BOOTSEL button, plug it in. Then you can release the button.
//! Mount the usb drive (this will be enumerated as USB mass storage) then run the following command:
//! cargo run --bin 05_button_send --release
//!
//! Troubleshoot:
//! `Error: "Unable to find mounted pico"`
//! This is because the pico is not in bootloader mode. You need to press down the BOOTSEL button when you plug it in and then release the button.
//! Then, if your're on linux, you need to mount the drive (click on it in your explorer and it should mount automatically).
//! Or run `run-automount.sh` to do it (see `.cargo//config.toml`)
//! Pressing CTRL+C or q in the terminal will terminate the program and restart the picow in boot mode

#![no_std]
#![no_main]

use core::str::FromStr;

use cyw43_pio::{PioSpi, DEFAULT_CLOCK_DIVIDER};
use embassy_executor::Spawner;
use embassy_net::{IpEndpoint, Ipv4Address};
use embassy_rp::{
    bind_interrupts,
    gpio::{Input, Level, Output, Pull},
    peripherals::{PIO0, USB},
    pio::{self, Pio},
    usb::{self},
};
use embassy_time::{Duration, Timer};
use log::info;
use rp_picow_examples::{
    self as _, logging::setup_logging, network::setup_network, radio::setup_radio,
};

bind_interrupts!(struct Irqs {
    USBCTRL_IRQ => usb::InterruptHandler<USB>;
    PIO0_IRQ_0 => pio::InterruptHandler<PIO0>;
});

#[embassy_executor::task]
async fn net_task(mut runner: embassy_net::Runner<'static, cyw43::NetDriver<'static>>) -> ! {
    runner.run().await
}

#[embassy_executor::main]
async fn main(spawner: Spawner) {
    const REMOTE_PORT: u16 = 47900;
    const LOCAL_PORT: u16 = 47901;
    let remote_ip =
        Ipv4Address::from_str(include_str!("../REMOTE_IP.txt")).expect("invalid remote ip address");
    let local_ip = Ipv4Address::from_str(include_str!("../LOCAL_IP.txt")).ok();

    let p = embassy_rp::init(Default::default());

    // setup logging over usb serial port
    let driver = usb::Driver::new(p.USB, Irqs);
    setup_logging(&spawner, driver);

    // wait for host to connect to usb serial port
    Timer::after(Duration::from_millis(1000)).await;
    info!("started");

    // setup spi bus for wifi modem
    let pwr = Output::new(p.PIN_23, Level::Low);
    let cs = Output::new(p.PIN_25, Level::High);
    let mut pio = Pio::new(p.PIO0, Irqs);
    let spi = PioSpi::new(
        &mut pio.common,
        pio.sm0,
        DEFAULT_CLOCK_DIVIDER,
        pio.irq0,
        cs,
        p.PIN_24,
        p.PIN_29,
        p.DMA_CH0,
    );
    let (net_device, mut control) = setup_radio(&spawner, pwr, spi).await;

    let socket = setup_network(&spawner, net_device, &mut control, local_ip, LOCAL_PORT).await;

    // this is GP15 (not the physical chip pin number!)
    let mut button = Input::new(p.PIN_15, Pull::Up);

    let remote_endpoint = IpEndpoint::new(remote_ip.into(), REMOTE_PORT);

    loop {
        info!("waiting for button press");
        button.wait_for_low().await;

        info!("send led on!");
        socket.send_to(b"on", remote_endpoint).await.unwrap();
        control.gpio_set(0, true).await;

        // debounce the button
        Timer::after(Duration::from_millis(100)).await;

        info!("waiting for button release");
        button.wait_for_high().await;

        info!("send led off!");
        socket.send_to(b"off", remote_endpoint).await.unwrap();
        control.gpio_set(0, false).await;

        // debounce the button
        Timer::after(Duration::from_millis(100)).await;
    }
}

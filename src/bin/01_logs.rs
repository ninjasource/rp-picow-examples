//! Start here. This example tests the RP Pico W USB serial port logging.
//!
//! How to run with a standard usb cable (no debug probe):
//! The pico has a builtin bootloader that can be used as a replacement for a debug probe (like an ST link v2).
//! Start with the usb cable unplugged then, while holding down the BOOTSEL button, plug it in. Then you can release the button.
//! Mount the usb drive (this will be enumerated as USB mass storage) then run the following command:
//! cargo run --bin 01_logs --release
//!
//!
//! Troubleshoot:
//! `Error: "Unable to find mounted pico"`
//! This is because the pico is not in bootloader mode. You need to press down the BOOTSEL button when you plug it in and then release the button.
//! Then, if your're on linux, you need to mount the drive (click on it in your explorer and it should mount automatically). 
//! Or run `run-automount.sh` to do it (see `.cargo//config.toml`)
//! Pressing CTRL+C or q in the terminal will terminate the program and restart the picow in boot mode

#![no_std]
#![no_main]

use embassy_executor::Spawner;
use embassy_rp::{
    bind_interrupts,
    peripherals::USB,
    usb::{self},
};
use embassy_time::{Duration, Timer};
use log::info;
use rp_picow_examples::{self as _, logging::setup_logging};

bind_interrupts!(struct Irqs {
    USBCTRL_IRQ => usb::InterruptHandler<USB>;
});

#[embassy_executor::main]
async fn main(spawner: Spawner) {
    let p = embassy_rp::init(Default::default());

    // setup logging over usb serial port
    let driver = usb::Driver::new(p.USB, Irqs);
    setup_logging(&spawner, driver);

    // wait for host to connect to usb serial port
    Timer::after(Duration::from_millis(1000)).await;
    info!("started");

    let mut counter = 0;
    loop {
        info!("county: {}", counter);
        counter += 1;
        Timer::after(Duration::from_secs(1)).await;
    }
}

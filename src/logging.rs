use core::str;

use embassy_executor::Spawner;
use embassy_rp::{peripherals::USB, rom_data::reset_to_usb_boot, usb::Driver};
use embassy_usb_logger::ReceiverHandler;

#[embassy_executor::task]
async fn logger_task(driver: Driver<'static, USB>) {
    embassy_usb_logger::run!(1024, log::LevelFilter::Info, driver, Handler);
}

pub fn setup_logging(spawner: &Spawner, driver: Driver<'static, USB>) {
    spawner.spawn(logger_task(driver)).unwrap();
}

pub struct Handler;

impl ReceiverHandler for Handler {
    async fn handle_data(&self, data: &[u8]) {
        if let Ok(data) = str::from_utf8(data) {
            let data = data.trim();

            // If you are using elf2uf2-term with the '-t' flag, then when closing the serial monitor,
            // this will automatically put the pico into boot mode
            if data == "q" || data == "elf2uf2-term" {
                reset_to_usb_boot(0, 0); // Restart the chip
            } else {
                log::info!("Recieved: {:?}", data);
            }
        }
    }

    fn new() -> Self {
        Self
    }
}

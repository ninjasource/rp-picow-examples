#![no_std]

pub mod logging;
pub mod network;
pub mod radio;

use embassy_rp::rom_data::reset_to_usb_boot;

#[panic_handler]
fn core_panic(_info: &core::panic::PanicInfo) -> ! {
    reset_to_usb_boot(0, 0);
    unreachable!()
}

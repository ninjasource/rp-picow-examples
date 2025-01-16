#![no_std]
#![no_main]

use defmt::*;
use embassy_executor::Spawner;
use embassy_time::{Duration, Timer};
//use {defmt_rtt as _, panic_probe as _};
use panic_halt as _;

#[embassy_executor::main]
async fn main(_spawner: Spawner) {
    let _p = embassy_rp::init(Default::default());

    // info!("started");

    let mut counter = 0;
    loop {
        //   info!("count: {}", counter);
        counter += 1;
        Timer::after(Duration::from_secs(1)).await;
    }
}

#![no_std]
#![no_main]

use defmt::info;
use defmt_rtt as _;
use panic_probe as _;

#[cortex_m_rt::entry]
fn main() -> ! {
    loop {}
}

#[embassy_executor::task]
async fn task() {
    info!("Starting...");
}

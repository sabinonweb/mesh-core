// #![no_std]
// #![no_main]
// #![feature(type_alias_impl_trait)]
//
// use embassy_executor::Spawner;
// use esp_hal::peripherals::Peripherals;
//
// #[main]
// async fn main(spawner: Spawner) {
//     let peripherals = Peripherals::take();
//     let system = peripherals.SYSTEM.split();
//
//     loop {}
// }

#![no_std]
#![no_main]

#[macro_use]
extern crate semihosting;
use semihosting::io::Writer;

use embassy_executor::Spawner;

#[embassy_executor::main]
async fn main(spawner: Spawner) {
    hprintln!("Hello").unwrap();

    loop {}
}

#![no_std]
#![no_main]

use cortex_m_rt::entry;
use embedded_hal::digital::OutputPin;
use mesh_core_no_std::hal::embassy_hal::GPIOState;
use panic_halt as _;

#[entry]
fn main() -> ! {
    let mut gpio = GPIOState::new();

    gpio.set_high().unwrap();
    gpio.set_low().unwrap();

    loop {}
}

#![deny(unsafe_code)]
#![no_main]
#![no_std]

use volatile_register::{RO, RW};

#[repr(C)]
struct SysTick {
    // Control and Status Register
    pub csr: RW<u32>,
    // Read Value Register
    pub rvr: RW<u32>,
    // Current Value Register
    pub cvr: RW<u32>,
    // Calibration Value Register
    pub calib: RO<u32>,
}

fn get_systick() -> &'static mut SysTick {
    unsafe { &mut *(0xE0000_E010 as *mut SysTick) }
}

fn get_time() -> u32 {
    let systick = get_systick();
    systick.cvr.read()
}

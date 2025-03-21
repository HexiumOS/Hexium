#![no_std]
#![feature(abi_x86_interrupt)]

extern crate alloc;

use core::arch::asm;

pub mod boot;
pub mod devices;
pub mod drivers;
pub mod interrupts;
pub mod log;
pub mod memory;
pub mod utils;
pub mod rtc;
pub mod writer;

pub fn init() {
    writer::init();
    interrupts::init();
    memory::init();

    info!("Hexium OS kernel v{} succesfully initialized at {}\n", env!("CARGO_PKG_VERSION"), unsafe { rtc::read_rtc() });
    info!("Welcome to Hexium OS\n");
}

pub fn hlt_loop() -> ! {
    loop {
        unsafe {
            #[cfg(target_arch = "x86_64")]
            asm!("hlt");
            #[cfg(any(target_arch = "aarch64", target_arch = "riscv64"))]
            asm!("wfi");
            #[cfg(target_arch = "loongarch64")]
            asm!("idle 0");
        }
    }
}

#![no_std]
#![feature(abi_x86_interrupt)]

use core::arch::asm;

pub mod boot;
pub mod devices;
pub mod drivers;
pub mod interrupts;
pub mod log;
pub mod memory;
pub mod utils;
pub mod writer;

pub fn init(framebuffer: limine::framebuffer::Framebuffer) {
    writer::init(framebuffer);
    interrupts::init();

    info!("Welcome to Infinity OS\n");
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

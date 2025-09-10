use crate::{info, println};

pub mod log;
pub mod panic;
pub mod print;

pub fn init() {
    init_x86_64();
}

fn init_x86_64() {
    crate::arch::interrupts::disable();
    assert!(crate::arch::boot::BASE_REVISION.is_supported());
    crate::arch::writer::init();
    info!("System initialization started");
    crate::arch::gdt::init();
    crate::arch::idt::init();
    println!();
    crate::arch::memory::pmm::init();
    println!();
    crate::arch::memory::heap::init();
    crate::arch::interrupts::enable();
}

pub fn halt() -> ! {
    loop {
        unsafe {
            core::arch::asm!("hlt");
        }
    }
}

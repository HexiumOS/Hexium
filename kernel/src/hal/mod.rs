use crate::{info, println, trace};

pub mod log;
pub mod panic;
pub mod print;

pub fn init() {
    init_x86_64();
}

fn init_x86_64() {
    /*
     * Early setup
     */
    crate::arch::interrupts::disable();
    assert!(crate::arch::boot::BASE_REVISION.is_supported());

    /*
     * Terminal Emulator
     */
    crate::arch::writer::init();
    info!("System initialization started");
    info!(
        "Boot revision {} supported",
        crate::arch::boot::BASE_REVISION.loaded_revision().unwrap()
    );

    /*
     * Descriptor Tables
     */
    println!();
    crate::arch::gdt::init();
    trace!("GDT initialized");
    crate::arch::idt::init();
    trace!("IDT initialized");

    /*
     * Memory Detetion
     */
    println!();
    crate::arch::memory::pmm::init();

    /*
     * Kernel Heap
     */
    println!();
    crate::arch::memory::heap::init();

    /*
     * Finished Setup
     */
    crate::arch::interrupts::enable();
    println!();
}

pub fn halt() -> ! {
    loop {
        unsafe {
            core::arch::asm!("hlt");
        }
    }
}

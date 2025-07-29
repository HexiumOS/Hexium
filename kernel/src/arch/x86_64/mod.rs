pub mod boot;
pub mod drivers;
pub mod gdt;
pub mod idt;
pub mod interrupts;
pub mod registers;
pub mod writer;

pub fn init() {
    interrupts::disable();
    assert!(boot::BASE_REVISION.is_supported());
    writer::init();
    gdt::init();
    idt::init();
    interrupts::enable();
}

#[repr(C, packed)]
struct DescriptorTablePointer {
    limit: u16,
    base: u64,
}

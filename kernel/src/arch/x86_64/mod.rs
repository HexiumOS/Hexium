pub mod boot;
pub mod drivers;
pub mod gdt;
pub mod interrupts;
pub mod registers;
pub mod writer;

pub fn init() {
    interrupts::disable();
    assert!(boot::BASE_REVISION.is_supported());
    writer::init();
    gdt::init();
    interrupts::enable();
}

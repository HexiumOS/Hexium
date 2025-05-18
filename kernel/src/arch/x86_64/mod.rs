pub mod interrupts;
pub mod limine;
pub mod memory;
pub mod registers;

pub fn init() {
    assert!(limine::BASE_REVISION.is_supported());
    crate::writer::init();
    interrupts::init();
    memory::init();
}

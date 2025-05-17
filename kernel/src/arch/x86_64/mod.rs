pub mod interrupts;
pub mod memory;
pub mod registers;

pub fn init() {
    crate::writer::init();
    interrupts::init();
    crate::memory::init();
}

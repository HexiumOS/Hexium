pub mod interrupts;

pub fn init() {
    crate::writer::init();
    interrupts::init();
    crate::memory::init();
}

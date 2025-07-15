pub mod print;

pub fn init() {
    crate::arch::init();
    crate::serial_println!("Initialized HexiumOS!");
}

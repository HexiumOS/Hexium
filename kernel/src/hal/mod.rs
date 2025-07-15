pub mod print;

pub fn init() {
    crate::arch::init();
    crate::println!("Initialized HexiumOS");
}

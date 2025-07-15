pub mod log;
pub mod print;

pub fn init() {
    crate::arch::init();
    crate::info!("Initialized HAL");
}

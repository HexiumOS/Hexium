pub mod log;
pub mod panic;
pub mod print;

pub fn init() {
    crate::arch::init();
}

pub fn halt() -> ! {
    loop {
        unsafe {
            core::arch::asm!("hlt");
        }
    }
}

pub mod log;
pub mod print;

pub fn init() {
    crate::arch::init();
}

pub fn halt_device() -> ! {
    loop {
        unsafe {
            core::arch::asm!("hlt");
        }
    }
}

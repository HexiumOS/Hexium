pub mod log;
pub mod print;

pub fn init() {
    crate::arch::init();
}

pub fn halt_device() -> ! {
    //unsafe {
    //    core::arch::asm!("cli");
    //}
    loop {
        unsafe {
            core::arch::asm!("hlt");
        }
    }
}

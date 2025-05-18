use crate::trace;

pub mod boot;
pub mod clock;
pub mod vfs;

pub fn init() {
    crate::arch::init();
    trace!("HAL initialized");
}

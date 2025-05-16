use crate::trace;

pub mod vfs;

pub fn init() {
    crate::arch::init();
    trace!("HAL initialized");
}

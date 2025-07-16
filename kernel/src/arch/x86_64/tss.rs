use crate::trace;
use lazy_static::lazy_static;
use x86_64c::{VirtAddr, instructions::tables::load_tss, structures::tss::TaskStateSegment};

pub const DOUBLE_FAULT_IST_INDEX: u16 = 0;

pub fn init() {
    unsafe {
        load_tss(super::gdt::GDT.1.tss_selector);
    }
    trace!("Initialized TSS");
}

lazy_static! {
    pub static ref TSS: TaskStateSegment = {
        let mut tss = TaskStateSegment::new();
        tss.interrupt_stack_table[DOUBLE_FAULT_IST_INDEX as usize] = {
            const STACK_SIZE: usize = 4096 * 5;
            static mut STACK: [u8; STACK_SIZE] = [0; STACK_SIZE];

            let stack_start = VirtAddr::from_ptr(&raw const STACK);
            let stack_end = stack_start + STACK_SIZE;
            stack_end
        };
        tss
    };
}

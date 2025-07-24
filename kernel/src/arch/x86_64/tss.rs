use crate::{
    arch::{addr::VirtAddr, registers::segmentation::SegmentSelector},
    trace,
};

pub fn init() {
    unsafe {
        load_tss(super::gdt::GDT.1.tss_selector);
    }
    trace!("Initialized TSS");
}

pub const DOUBLE_FAULT_IST_INDEX: u16 = 0;

lazy_static::lazy_static! {
    pub static ref TSS: TaskStateSegment = {
        let mut tss = TaskStateSegment::new();
        tss.interrupt_stack_table[DOUBLE_FAULT_IST_INDEX as usize] = {
            const STACK_SIZE: usize = 4096 * 5;
            static mut STACK: [u8; STACK_SIZE] = [0; STACK_SIZE];

            let stack_start = VirtAddr::from_ptr(&raw const STACK);
            let stack_end = stack_start + STACK_SIZE.try_into().unwrap();
            stack_end
        };
        tss
    };
}

#[inline]
pub unsafe fn load_tss(sel: SegmentSelector) {
    unsafe {
        core::arch::asm!("ltr {0:x}", in(reg) sel.0, options(nostack, preserves_flags));
    }
}

#[repr(C, packed(4))]
pub struct TaskStateSegment {
    _reserved_1: u32,
    pub privilege_stack_table: [VirtAddr; 3],
    _reserved_2: u64,
    pub interrupt_stack_table: [VirtAddr; 7],
    _reserved_3: u64,
    _reserved_4: u16,
    pub iomap_base: u16,
}

impl TaskStateSegment {
    #[inline]
    pub const fn new() -> TaskStateSegment {
        TaskStateSegment {
            _reserved_1: 0,
            privilege_stack_table: [VirtAddr::zero(); 3],
            _reserved_2: 0,
            interrupt_stack_table: [VirtAddr::zero(); 7],
            _reserved_3: 0,
            _reserved_4: 0,
            iomap_base: size_of::<TaskStateSegment>() as u16,
        }
    }
}

impl Default for TaskStateSegment {
    #[inline]
    fn default() -> Self {
        Self::new()
    }
}

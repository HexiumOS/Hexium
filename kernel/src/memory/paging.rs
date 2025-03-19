use crate::{error, hlt_loop, print};
use x86_64::structures::idt::{InterruptStackFrame, PageFaultErrorCode};
use x86_64::{
    VirtAddr,
    structures::paging::{PageTable, OffsetPageTable}
};

pub fn init() -> OffsetPageTable<'static> {
    let level_4_table = unsafe { active_level_4_table(super::phys_mem_offset()) };
    unsafe { OffsetPageTable::new(level_4_table, super::phys_mem_offset()) }
}

pub unsafe fn active_level_4_table(_physical_memory_offset: VirtAddr) -> &'static mut PageTable {
    use x86_64::registers::control::Cr3;

    let (level_4_table_frame, _) = Cr3::read();

    let phys = level_4_table_frame.start_address();
    let virt = super::phys_mem_offset() + phys.as_u64();
    let page_table_ptr: *mut PageTable = virt.as_mut_ptr();

    unsafe { &mut *page_table_ptr }
}

pub extern "x86-interrupt" fn page_fault_handler(
    stack_frame: InterruptStackFrame,
    error_code: PageFaultErrorCode,
) {
    use x86_64::registers::control::Cr2;

    error!("EXCEPTION: PAGE FAULT\n");
    error!("Accessed Address: {:?}\n", Cr2::read());
    error!("Error Code: {:?}\n", error_code);
    print!("\n{:#?}\n", stack_frame);
    hlt_loop();
}

use crate::{error, hlt_loop, print};
use x86_64c::structures::{
    idt::{InterruptStackFrame, PageFaultErrorCode},
    paging::OffsetPageTable,
};

pub extern "x86-interrupt" fn page_fault_handler(
    stack_frame: InterruptStackFrame,
    error_code: PageFaultErrorCode,
) {
    use x86_64c::registers::control::Cr2;

    error!("EXCEPTION: PAGE FAULT");
    error!("Accessed Address: {:?}", Cr2::read());
    error!("Error Code: {:?}", error_code);
    print!("\n{:#?}\n", stack_frame);
    hlt_loop();
}

pub fn initialize_offset_table() -> OffsetPageTable<'static> {
    unsafe {
        let level_4_table = super::paging::active_level_4_table();
        OffsetPageTable::new(level_4_table, super::hhdm_offset())
    }
}

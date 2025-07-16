use super::tss;
use crate::{debug, trace};
use lazy_static::lazy_static;
use x86_64c::structures::idt::{InterruptDescriptorTable, InterruptStackFrame};

lazy_static! {
    static ref IDT: InterruptDescriptorTable = {
        let mut idt = InterruptDescriptorTable::new();
        idt.breakpoint.set_handler_fn(breakpoint_handler);
        unsafe {
            idt.double_fault
                .set_handler_fn(double_fault_handler)
                .set_stack_index(tss::DOUBLE_FAULT_IST_INDEX);
        }
        idt
    };
}

pub fn init() {
    IDT.load();
    trace!("Initialized IDT");
}

extern "x86-interrupt" fn breakpoint_handler(stack_frame: InterruptStackFrame) {
    debug!("EXCEPTION: BREAKPOINT\n{:#?}", stack_frame);
}

extern "x86-interrupt" fn double_fault_handler(
    stack_frame: InterruptStackFrame,
    _error_code: u64,
) -> ! {
    panic!("EXCEPTION: DOUBLE FAULT\n{:#?}", stack_frame);
}

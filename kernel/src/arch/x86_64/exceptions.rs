use crate::{arch::idt::InterruptStackFrame, panic::kpanic};

pub extern "x86-interrupt" fn double_fault_handler(
    stack_frame: InterruptStackFrame,
    error_code: u64,
) -> ! {
    kpanic("Double Fault", error_code, Some(stack_frame));
}

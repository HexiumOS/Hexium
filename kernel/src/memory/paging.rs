use crate::{error, hlt_loop, print};
use x86_64::structures::idt::{InterruptStackFrame, PageFaultErrorCode};

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

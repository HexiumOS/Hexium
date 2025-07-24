use crate::{
    arch::{
        drivers::pic8259::{InterruptIndex, PICS},
        idt::InterruptStackFrame,
    },
    print,
};

pub extern "x86-interrupt" fn interrupt_handler(_stack_frame: InterruptStackFrame) {
    print!(".");
    PICS.lock().send_eoi(InterruptIndex::Timer.as_u8());
}

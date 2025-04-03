use crate::trace;
use x86_64::structures::idt::InterruptStackFrame;
use crate::interrupts::InterruptIndex;
use x86_64::instructions::port::Port;

pub extern "x86-interrupt" fn interrupt_handler(_stack_frame: InterruptStackFrame) {
    let mut keyboard_port: Port<u8> = Port::new(0x60);
    
    // Read the scancode from the keyboard data port
    let scancode: u8 = unsafe { keyboard_port.read() };
    
    trace!("Received keyboard interrupt with scancode: {:#x}\n", scancode);

    unsafe {
        crate::interrupts::PICS.lock()
            .notify_end_of_interrupt(InterruptIndex::Keyboard.as_u8());
    }
}

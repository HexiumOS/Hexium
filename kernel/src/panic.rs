use crate::arch::debug::stacktrace::print_stacktrace;
use crate::arch::registers::*;
use crate::{arch::idt::InterruptStackFrame, hal::halt, print, println};

const BACKGROUND: &str = "\x1b[41m";
const CLEAR_SCREEN: &str = "\x1b[2J";
const HIDE_CURSOR: &str = "\x1b[?25l";
const CURSOR_HOME: &str = "\x1b[H";

/// Handles a kernel panic.
///
/// Arguments:
/// - `message`: Short info message that will be printed
/// - `error_code`: Error code that will be printed. 0 means unknown
/// - `stack_frame`: Optional interrupt stack frame with additional information in case of a diverging interrupt
pub fn kpanic(message: &str, error_code: u64, stack_frame: Option<InterruptStackFrame>) -> ! {
    print!(
        "{}{}{}{}",
        BACKGROUND, CLEAR_SCREEN, CURSOR_HOME, HIDE_CURSOR
    );
    println!("--- KERNEL PANIC ---");

    println!("Message:    {}", message);
    println!("Error Code: {:#x}", error_code);
    println!();
    println!("Register Dump:");
    print_register_dump(&get_registers());
    println!();
    println!("Stacktrace:");
    print_stacktrace();

    halt();
}

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
pub fn kpanic(message: &str, error_code: u64, _stack_frame: Option<InterruptStackFrame>) -> ! {
    print!(
        "{}{}{}{}",
        BACKGROUND, CLEAR_SCREEN, CURSOR_HOME, HIDE_CURSOR
    );
    println!("--- KERNEL PANIC ---");

    print_overview(message, error_code);
    println!();
    println!("Register Dump:");
    print_register_dump(&get_registers());
    println!();
    println!("Stacktrace:");
    print_stacktrace();

    halt();
}

fn print_overview(message: &str, error_code: u64) {
    println!("Message:    {}", message);
    println!("Error code: {:#X}", error_code);
    match message {
        "General Protection Fault" => {
            if error_code == 0 {
                println!(
                    "No segment selector involved (e.g., privilege violation or invalid operation)"
                );
            } else {
                let table = match error_code & 0x3 {
                    0 => "GDT",
                    1 => "IDT",
                    2 => "LDT",
                    3 => "IDT",
                    _ => unreachable!(),
                };
                let external = if (error_code & 0x8) != 0 {
                    "external"
                } else {
                    "kernel"
                };
                let index = (error_code >> 3) & 0x1FFF;
                println!(
                    "Segment selector error: table={}, index={:#x}, {}",
                    table, index, external
                );
            }
        }
        "Page Fault" => {
            let present = if (error_code & 0x1) != 0 {
                "protection violation"
            } else {
                "non-present page"
            };
            let write = if (error_code & 0x2) != 0 {
                "write"
            } else {
                "read"
            };
            let user = if (error_code & 0x4) != 0 {
                "user mode"
            } else {
                "supervisor mode"
            };
            let reserved = if (error_code & 0x8) != 0 {
                ", reserved bit violation"
            } else {
                ""
            };
            let instruction = if (error_code & 0x10) != 0 {
                ", instruction fetch"
            } else {
                ""
            };
            let protection_key = if (error_code & 0x20) != 0 {
                ", protection key violation"
            } else {
                ""
            };
            let sgx = if (error_code & 0x8000) != 0 {
                ", SGX violation"
            } else {
                ""
            };
            let faulting_address = unsafe { get_cr2() }; // Assume get_cr2() returns the CR2 register value
            println!(
                "Details:    Caused by {} on {} in {}{}{}{}{}, faulting address: {:#x}",
                present, write, user, reserved, instruction, protection_key, sgx, faulting_address
            );
        }
        _ => {}
    }
}

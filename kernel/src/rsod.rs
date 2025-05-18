use crate::arch::debug::stacktrace::print_stack_trace;
use crate::arch::registers::{get_registers, print_register_dump_with_color};
use crate::print;

macro_rules! panic_log {
    ($($arg:tt)*) => {
        print!("\x1b[41m\x1b[37m"); // Red background, white text
        print!($($arg)*);
        print!("\x1b[0m"); // Reset colors
    };
}

pub fn rsod_handler(info: &core::panic::PanicInfo) -> ! {
    clear_screen_red();
    panic_log!("{}\n", info);
    print_register_dump_with_color(&get_registers(), "37", "41");
    unsafe { print_stack_trace() };
    loop {}
}

pub fn clear_screen_red() {
    const RED_BACKGROUND: &str = "\x1b[41m"; // ANSI code for red background
    const RESET: &str = "\x1b[0m"; // ANSI code to reset colors
    const CLEAR_SCREEN: &str = "\x1b[2J"; // ANSI code to clear the screen
    const CURSOR_HOME: &str = "\x1b[H"; // ANSI code to move cursor to the top-left corner

    print!("{}{}{}{}", RED_BACKGROUND, CLEAR_SCREEN, CURSOR_HOME, RESET);
}

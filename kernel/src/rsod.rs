/*
 * This file is part of Hexium OS.
 * Copyright (C) 2025 The Hexium OS Authors – see the AUTHORS file.
 *
 * This program is free software: you can redistribute it and/or modify
 * it under the terms of the GNU General Public License as published by
 * the Free Software Foundation, either version 3 of the License, or
 * (at your option) any later version.
 *
 * This program is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
 * GNU General Public License for more details.
 *
 * You should have received a copy of the GNU General Public License
 * along with this program. If not, see <https://www.gnu.org/licenses/>.
 */

use crate::debug::stacktrace::print_stack_trace;
use crate::print;

macro_rules! panic_log {
    ($($arg:tt)*) => {
        print!("\x1b[41m\x1b[37m"); // Red background, white text
        print!($($arg)*);
        print!("\x1b[0m"); // Reset colors
    };
}

pub fn rsod_handler(info: &core::panic::PanicInfo) -> ! {
    use crate::utils::registers::{get_registers, print_register_dump_with_color};
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

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

#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(hexium_os::test_runner)]
#![reexport_test_harness_main = "test_main"]

use core::panic::PanicInfo;

use hexium_os::{init, println};

#[unsafe(no_mangle)]
unsafe extern "C" fn kmain() -> ! {
    init(); // Issue#30: Not sure why it's absence causes an loop running of test_println_long test.
    test_main();
    loop {}
}

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    hexium_os::test_panic_handler(info);
}

#[test_case]
fn test_println_simple() {
    println!("Simple print new line statement");
}

#[test_case]
fn test_println_long() {
    for _ in 0..200 {
        println!("Simple print new line many times");
    }
}

#[test_case]
fn test_println_long_more() {
    for _ in 0..200 {
        println!("Simple print new line many times");
    }
}

// TODO: Issue#31: Needs buffer access
// #[test_case]
// fn test_println_output() {
//     let s = "Some test fitting single line";
//     println!("{}", s);
//     for (i, c) in s.chars().enumerate() {
//         let screen_char = WRITER.lock().write_char(c).buffer.chars[BUFFER_HEIGHT - 2][i].read();
//         assert_eq!(char::from(screen_char.ascii_character), c);
//     }
// }

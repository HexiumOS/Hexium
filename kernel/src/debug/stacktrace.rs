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

use crate::print;

#[repr(C)]
struct StackFrame {
    prev_frame: *const StackFrame,
    return_address: usize,
}

pub unsafe fn print_stack_trace() {
    crate::println!("\n\x1b[37;41mStacktrace:");

    let mut frame: *const StackFrame;

    unsafe {
        core::arch::asm!(
            "mov {}, rbp",
            out(reg) frame,
            options(nostack, nomem, preserves_flags)
        );
    }

    let mut depth = 0;

    while let Some(f) = unsafe { frame.as_ref() } {
        print!(
            "Frame {}: return address = 0x{:016X}\n",
            depth, f.return_address
        );
        frame = f.prev_frame;
        depth += 1;

        if depth > 64 {
            print!("Stack trace aborted (too deep)\n");
            break;
        }
    }
}

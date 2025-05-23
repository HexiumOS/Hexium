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

use core::arch::asm;

/// Returns the registers of the CPU in a struct
pub fn get_registers() -> Registers {
    let (rax, rbx, rcx, rdx): (u64, u64, u64, u64);
    let (rsi, rdi, rsp, rbp): (u64, u64, u64, u64);
    let (r8, r9, r10, r11): (u64, u64, u64, u64);
    let (r12, r13, r14, r15): (u64, u64, u64, u64);
    let rip: u64;
    let rflags: u64;
    let (cs, ds, es, fs, gs, ss): (u16, u16, u16, u16, u16, u16);

    unsafe {
        // First group
        asm!(
            "mov {0}, rax",
            "mov {1}, rbx",
            "mov {2}, rcx",
            "mov {3}, rdx",
            out(reg) rax,
            out(reg) rbx,
            out(reg) rcx,
            out(reg) rdx,
            options(nomem, nostack),
        );

        // Second group
        asm!(
            "mov {0}, rsi",
            "mov {1}, rdi",
            "mov {2}, rsp",
            "mov {3}, rbp",
            out(reg) rsi,
            out(reg) rdi,
            out(reg) rsp,
            out(reg) rbp,
            options(nomem, nostack),
        );

        // Third group
        asm!(
            "mov {0}, r8",
            "mov {1}, r9",
            "mov {2}, r10",
            "mov {3}, r11",
            out(reg) r8,
            out(reg) r9,
            out(reg) r10,
            out(reg) r11,
            options(nomem, nostack),
        );

        // Fourth group
        asm!(
            "mov {0}, r12",
            "mov {1}, r13",
            "mov {2}, r14",
            "mov {3}, r15",
            out(reg) r12,
            out(reg) r13,
            out(reg) r14,
            out(reg) r15,
            options(nomem, nostack),
        );

        // RIP (instruction pointer)
        asm!(
            "lea {0}, [rip]",
            out(reg) rip,
            options(nomem, nostack),
        );

        // RFLAGS
        asm!(
            "pushfq",
            "pop {0}",
            out(reg) rflags,
            options(nomem),
        );

        // Segment Registers
        asm!(
            "mov {0:x}, cs",
            "mov {1:x}, ds",
            "mov {2:x}, es",
            "mov {3:x}, fs",
            "mov {4:x}, gs",
            "mov {5:x}, ss",
            out(reg) cs,
            out(reg) ds,
            out(reg) es,
            out(reg) fs,
            out(reg) gs,
            out(reg) ss,
            options(nomem, nostack),
        );
    }

    Registers {
        rax,
        rbx,
        rcx,
        rdx,
        rsi,
        rdi,
        rsp,
        rbp,
        rip,
        r8,
        r9,
        r10,
        r11,
        r12,
        r13,
        r14,
        r15,
        rflags,
        cs,
        ds,
        es,
        fs,
        gs,
        ss,
    }
}
#[derive(Debug, Default)]
pub struct Registers {
    // General Purpose
    pub rax: u64,
    pub rbx: u64,
    pub rcx: u64,
    pub rdx: u64,
    pub rsi: u64,
    pub rdi: u64,
    pub rbp: u64,
    pub rsp: u64,
    pub r8: u64,
    pub r9: u64,
    pub r10: u64,
    pub r11: u64,
    pub r12: u64,
    pub r13: u64,
    pub r14: u64,
    pub r15: u64,

    // Control Registers
    pub rip: u64,
    pub rflags: u64,

    // Segment Registers
    pub cs: u16,
    pub ds: u16,
    pub es: u16,
    pub fs: u16,
    pub gs: u16,
    pub ss: u16,
}

// Helper macros for formatting
macro_rules! print_pair {
    ($left:expr, $left_val:expr, $right:expr, $right_val:expr) => {
        println!(
            "│ {:.<4} 0x{:016x} │ {:.<4} 0x{:016x} │",
            $left, $left_val, $right, $right_val
        );
    };
}

macro_rules! print_segment_pair {
    ($left:expr, $left_val:expr, $right:expr, $right_val:expr) => {
        println!(
            "│ {:<4} 0x{:04x}             │ {:<4} 0x{:04x}             │",
            $left, $left_val, $right, $right_val
        );
    };
}

macro_rules! print_pair_with_color {
    ($left:expr, $left_val:expr, $right:expr, $right_val:expr, $fg_color:expr, $bg_color:expr) => {
        println!(
            "\x1b[{};{}m│ {:.<4} 0x{:016x} │ {:.<4} 0x{:016x} │",
            $fg_color, $bg_color, $left, $left_val, $right, $right_val
        );
    };
}

macro_rules! print_segment_pair_with_color {
    ($left:expr, $left_val:expr, $right:expr, $right_val:expr, $fg_color:expr, $bg_color:expr) => {
        println!(
            "\x1b[{};{}m│ {:<4} 0x{:04x}             │ {:<4} 0x{:04x}             │",
            $fg_color, $bg_color, $left, $left_val, $right, $right_val
        );
    };
}

pub fn print_register_dump_with_color(regs: &Registers, fg_color: &str, bg_color: &str) {
    use crate::println;
    println!("\x1b[{};{}m\nRegister Dump:", fg_color, bg_color);
    println!(
        "\x1b[{};{}m┌─────────────────────────┬─────────────────────────┐",
        fg_color, bg_color
    );

    // General Purpose Registers
    print_pair_with_color!("rax", regs.rax, "rbx", regs.rbx, fg_color, bg_color);
    print_pair_with_color!("rcx", regs.rcx, "rdx", regs.rdx, fg_color, bg_color);
    print_pair_with_color!("rsi", regs.rsi, "rdi", regs.rdi, fg_color, bg_color);
    print_pair_with_color!("rbp", regs.rbp, "rsp", regs.rsp, fg_color, bg_color);
    print_pair_with_color!("r8 ", regs.r8, "r9 ", regs.r9, fg_color, bg_color);
    print_pair_with_color!("r10", regs.r10, "r11", regs.r11, fg_color, bg_color);
    print_pair_with_color!("r12", regs.r12, "r13", regs.r13, fg_color, bg_color);
    print_pair_with_color!("r14", regs.r14, "r15", regs.r15, fg_color, bg_color);

    println!(
        "\x1b[{};{}m├─────────────────────────┴─────────────────────────┤",
        fg_color, bg_color
    );

    // Control Registers
    println!(
        "\x1b[{};{}m│ {: <30} 0x{:016x} │",
        fg_color, bg_color, "rip:", regs.rip
    );
    println!(
        "\x1b[{};{}m│ {: <30} 0x{:016x} │",
        fg_color, bg_color, "rflags:", regs.rflags
    );

    println!(
        "\x1b[{};{}m├─────────────────────────┬─────────────────────────┤",
        fg_color, bg_color
    );

    // Segment Registers
    print_segment_pair_with_color!("cs", regs.cs, "ds", regs.ds, fg_color, bg_color);
    print_segment_pair_with_color!("es", regs.es, "fs", regs.fs, fg_color, bg_color);
    print_segment_pair_with_color!("gs", regs.gs, "ss", regs.ss, fg_color, bg_color);

    println!(
        "\x1b[{};{}m└─────────────────────────┴─────────────────────────┘\x1b[0m",
        fg_color, bg_color
    );
}

pub fn print_register_dump(regs: &Registers) {
    use crate::println;
    println!("\nRegister Dump:");
    println!("┌─────────────────────────┬─────────────────────────┐");

    // General Purpose Registers
    print_pair!("rax", regs.rax, "rbx", regs.rbx);
    print_pair!("rcx", regs.rcx, "rdx", regs.rdx);
    print_pair!("rsi", regs.rsi, "rdi", regs.rdi);
    print_pair!("rbp", regs.rbp, "rsp", regs.rsp);
    print_pair!("r8 ", regs.r8, "r9 ", regs.r9);
    print_pair!("r10", regs.r10, "r11", regs.r11);
    print_pair!("r12", regs.r12, "r13", regs.r13);
    print_pair!("r14", regs.r14, "r15", regs.r15);

    println!("├─────────────────────────┴─────────────────────────┤");

    // Control Registers
    println!("│ {: <30} 0x{:016x} │", "rip:", regs.rip);
    println!("│ {: <30} 0x{:016x} │", "rflags:", regs.rflags);

    println!("├─────────────────────────┬─────────────────────────┤");

    // Segment Registers
    print_segment_pair!("cs", regs.cs, "ds", regs.ds);
    print_segment_pair!("es", regs.es, "fs", regs.fs);
    print_segment_pair!("gs", regs.gs, "ss", regs.ss);

    println!("└─────────────────────────┴─────────────────────────┘");
}

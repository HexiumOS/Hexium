use core::arch::asm;
use core::fmt;

/// Returns the registers of the CPU in a struct
pub fn get_registers() -> Registers {
    let (rax, rbx, rcx, rdx): (u64, u64, u64, u64);
    let (rsi, rdi, rsp, rbp): (u64, u64, u64, u64);
    let (r8, r9, r10, r11): (u64, u64, u64, u64);
    let (r12, r13, r14, r15): (u64, u64, u64, u64);
    let rip: u64;
    let rflags: u64;

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
    }
}

/// Struct to hold the register values.
#[derive(Debug)]
pub struct Registers {
    rax: u64,
    rbx: u64,
    rcx: u64,
    rdx: u64,
    rsi: u64,
    rdi: u64,
    rsp: u64,
    rbp: u64,
    rip: u64,
    r8: u64,
    r9: u64,
    r10: u64,
    r11: u64,
    r12: u64,
    r13: u64,
    r14: u64,
    r15: u64,
    rflags: u64,
}

/// Display trait to format the Registers struct nicely.
impl fmt::Display for Registers {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "Registers:")?;
        writeln!(f, "  rax: {:016x}", self.rax)?;
        writeln!(f, "  rbx: {:016x}", self.rbx)?;
        writeln!(f, "  rcx: {:016x}", self.rcx)?;
        writeln!(f, "  rdx: {:016x}", self.rdx)?;
        writeln!(f, "  rsi: {:016x}", self.rsi)?;
        writeln!(f, "  rdi: {:016x}", self.rdi)?;
        writeln!(f, "  rsp: {:016x}", self.rsp)?;
        writeln!(f, "  rbp: {:016x}", self.rbp)?;
        writeln!(f, "  rip: {:016x}", self.rip)?;
        writeln!(f, "  r8:  {:016x}", self.r8)?;
        writeln!(f, "  r9:  {:016x}", self.r9)?;
        writeln!(f, "  r10: {:016x}", self.r10)?;
        writeln!(f, "  r11: {:016x}", self.r11)?;
        writeln!(f, "  r12: {:016x}", self.r12)?;
        writeln!(f, "  r13: {:016x}", self.r13)?;
        writeln!(f, "  r14: {:016x}", self.r14)?;
        writeln!(f, "  r15: {:016x}", self.r15)?;
        writeln!(f, "  rflags: {:016x}", self.rflags)?;
        Ok(())
    }
}

/// Function to print the register dump.
pub fn print_register_dump(registers: &Registers) {
    crate::print!("{}\n", registers);
}

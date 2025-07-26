use bitflags::bitflags;
use core::arch::asm;

bitflags! {
    #[repr(transparent)]
    #[derive(PartialEq, Eq, PartialOrd, Ord, Hash, Debug, Clone, Copy)]
    pub struct RFlags: u64 {
        const ID = 1 << 21;
        const VIRTUAL_INTERRUPT_PENDING = 1 << 20;
        const VIRTUAL_INTERRUPT = 1 << 19;
        const ALIGNMENT_CHECK = 1 << 18;
        const VIRTUAL_8086_MODE = 1 << 17;
        const RESUME_FLAG = 1 << 16;
        const NESTED_TASK = 1 << 14;
        const IOPL_HIGH = 1 << 13;
        const IOPL_LOW = 1 << 12;
        const OVERFLOW_FLAG = 1 << 11;
        const DIRECTION_FLAG = 1 << 10;
        const INTERRUPT_FLAG = 1 << 9;
        const TRAP_FLAG = 1 << 8;
        const SIGN_FLAG = 1 << 7;
        const ZERO_FLAG = 1 << 6;
        const AUXILIARY_CARRY_FLAG = 1 << 4;
        const PARITY_FLAG = 1 << 2;
        const CARRY_FLAG = 1;
    }
}

#[inline]
pub fn read() -> RFlags {
    RFlags::from_bits_truncate(read_raw())
}

#[inline]
pub fn read_raw() -> u64 {
    let raw: u64;

    unsafe {
        asm!("pushfq; pop {}", out(reg) raw, options(nomem, preserves_flags));
    }

    raw
}

#[inline]
pub unsafe fn write(flags: RFlags) {
    let old_value = read_raw();
    let reserved = old_value & !(RFlags::all().bits());
    let new_value = reserved | flags.bits();

    unsafe {
        write_raw(new_value);
    }
}

#[inline]
pub unsafe fn write_raw(val: u64) {
    unsafe {
        asm!("push {}; popfq", in(reg) val, options(nomem, preserves_flags));
    }
}

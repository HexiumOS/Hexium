use core::arch::asm;

#[inline]
pub fn are_enabled() -> bool {
    use super::registers::rflags::{self, RFlags};

    rflags::read().contains(RFlags::INTERRUPT_FLAG)
}

#[inline]
pub fn without_interrupts<F, R>(f: F) -> R
where
    F: FnOnce() -> R,
{
    let saved_int_flag = are_enabled();

    if saved_int_flag {
        disable();
    }

    let returns: R = f();

    if saved_int_flag {
        enable();
    }

    returns
}

pub fn enable() {
    unsafe {
        asm!("sti", options(preserves_flags, nostack));
    }
}

pub fn disable() {
    unsafe {
        asm!("cli", options(preserves_flags, nostack));
    }
}

pub fn wait() {
    unsafe {
        asm!("hlt", options(preserves_flags, nostack));
    }
}

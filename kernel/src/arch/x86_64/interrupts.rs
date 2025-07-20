use core::arch::asm;

#[inline]
pub fn are_enabled() -> bool {
    use super::registers::rflags::{self, RFlags};

    rflags::read().contains(RFlags::INTERRUPT_FLAG)
}

#[inline]
pub fn enable() {
    // Omit `nomem` to imitate a lock release. Otherwise, the compiler
    // is free to move reads and writes through this asm block.
    unsafe {
        asm!("sti", options(preserves_flags, nostack));
    }
}

#[inline]
pub fn disable() {
    // Omit `nomem` to imitate a lock acquire. Otherwise, the compiler
    // is free to move reads and writes through this asm block.
    unsafe {
        asm!("cli", options(preserves_flags, nostack));
    }
}

#[inline]
pub fn without_interrupts<F, R>(f: F) -> R
where
    F: FnOnce() -> R,
{
    let saved_intpt_flag = are_enabled();

    if saved_intpt_flag {
        disable();
    }

    let returns = f();

    if saved_intpt_flag {
        enable();
    }

    returns
}

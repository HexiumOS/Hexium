pub mod rflags;

#[inline]
pub fn get_cs() -> u16 {
    let segment: u16;
    unsafe {
        core::arch::asm!("mov {0:x}, cs",
            out(reg)
            segment,
            options(nomem, preserves_flags, nostack));
    }
    segment
}

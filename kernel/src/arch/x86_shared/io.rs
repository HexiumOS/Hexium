#[macro_export]
macro_rules! outb {
    ($port:expr, $value:expr) => {{
        core::arch::asm!(
            "out dx, al",
            in("dx") $port,
            in("al") $value,
            options(nomem, nostack, preserves_flags)
        );
    }};
}

#[macro_export]
macro_rules! inb {
    ($port:expr) => {{
        let result: u8;
        core::arch::asm!(
            "in al, dx",
            in("dx") $port,
            out("al") result,
            options(nomem, nostack, preserves_flags)
        );
        result
    }};
}

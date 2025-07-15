/// Prints to the host through the serial interface.
#[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
#[macro_export]
macro_rules! serial_print {
    ($($arg:tt)*) => {
        $crate::arch::serial::_print(format_args!($($arg)*));
    };
}

/// Prints to the host through the serial interface, appending a newline.
#[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
#[macro_export]
macro_rules! serial_println {
    () => ($crate::serial_print!("\n"));
    ($fmt:expr) => ($crate::serial_print!(concat!($fmt, "\n")));
    ($fmt:expr, $($arg:tt)*) => ($crate::serial_print!(
        concat!($fmt, "\n"), $($arg)*));
}

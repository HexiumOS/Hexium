#[derive(Debug, Clone, Copy, PartialEq)]
pub enum LogLevel {
    None = 0,
    Trace,
    Debug,
    Info,
    Warn,
    Error,
    Fatal,
    Panic,
}

// Define a macro that accepts the log level and message.
#[macro_export]
macro_rules! log {
    // Log level and a formatted message
    ($level:expr, $($arg:tt)*) => {{
        let (label, color_code) = match $level {
            $crate::LogLevel::None => ("NONE ", "\x1b[90m"),  // Bright Black (Gray), if needed
            $crate::LogLevel::Trace => ("TRACE", "\x1b[95m"), // Bright Magenta
            $crate::LogLevel::Debug => ("DEBUG", "\x1b[94m"), // Bright Blue
            $crate::LogLevel::Info => ("INFO ", "\x1b[92m"),  // Bright Green
            $crate::LogLevel::Warn => ("WARN ", "\x1b[93m"),  // Bright Yellow
            $crate::LogLevel::Error => ("ERROR", "\x1b[91m"), // Bright Red
            $crate::LogLevel::Fatal => ("FATAL", "\x1b[91m"), // Bright Red (same as ERROR)
            $crate::LogLevel::Panic => ("PANIC", "\x1b[97;41m"), // White text on Red background
        };

        $crate::print!("{}[{}]\x1b[0m {}", color_code, label, format_args!($($arg)*));
    }};
}

#[macro_export]
macro_rules! trace {
    ($($arg:tt)*) => {
        $crate::log!($crate::LogLevel::Trace, $($arg)*);
    };
}

#[macro_export]
macro_rules! debug {
    ($($arg:tt)*) => {
        #[cfg(not(release))]
        {
            $crate::log!($crate::LogLevel::Debug, $($arg)*);
        }
        
        #[cfg(release)]
        {}
    };
}

#[macro_export]
macro_rules! info {
    ($($arg:tt)*) => {
        $crate::log!($crate::LogLevel::Info, $($arg)*);
    };
}

#[macro_export]
macro_rules! warn {
    ($($arg:tt)*) => {
        $crate::log!($crate::LogLevel::Warn, $($arg)*);
    };
}

#[macro_export]
macro_rules! error {
    ($($arg:tt)*) => {
        $crate::log!($crate::LogLevel::Error, $($arg)*);
    };
}

#[macro_export]
macro_rules! fatal {
    ($($arg:tt)*) => {
        $crate::log!($crate::LogLevel::Fatal, $($arg)*);
    };
}

#[macro_export]
macro_rules! panic_log {
    ($($arg:tt)*) => {
        $crate::log!($crate::LogLevel::Panic, $($arg)*);
    };
}

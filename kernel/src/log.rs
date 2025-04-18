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
            $crate::log::LogLevel::None => ("NONE ", "\x1b[90m"),  // Bright Black (Gray), if needed
            $crate::log::LogLevel::Trace => ("TRACE", "\x1b[95m"), // Bright Magenta
            $crate::log::LogLevel::Debug => ("DEBUG", "\x1b[94m"), // Bright Blue
            $crate::log::LogLevel::Info => ("INFO ", "\x1b[92m"),  // Bright Green
            $crate::log::LogLevel::Warn => ("WARN ", "\x1b[93m"),  // Bright Yellow
            $crate::log::LogLevel::Error => ("ERROR", "\x1b[91m"), // Bright Red
            $crate::log::LogLevel::Fatal => ("FATAL", "\x1b[91m"), // Bright Red (same as ERROR)
            $crate::log::LogLevel::Panic => ("PANIC", "\x1b[97;41m"), // White text on Red background
        };

        $crate::println!("{}[{}]\x1b[0m {}", color_code, label, format_args!($($arg)*));
    }};
}

#[macro_export]
macro_rules! trace {
    ($($arg:tt)*) => {
        $crate::log!($crate::log::LogLevel::Trace, $($arg)*)
    };
}

#[macro_export]
macro_rules! debug {
    ($($arg:tt)*) => {
        #[cfg(debug_assertions)]
        {
            $crate::log!($crate::log::LogLevel::Debug, $($arg)*);
        }
        
        #[cfg(not(debug_assertions))]
        {}
    };
}

#[macro_export]
macro_rules! info {
    ($($arg:tt)*) => {
        $crate::log!($crate::log::LogLevel::Info, $($arg)*);
    };
}

#[macro_export]
macro_rules! warn {
    ($($arg:tt)*) => {
        $crate::log!($crate::log::LogLevel::Warn, $($arg)*);
    };
}

#[macro_export]
macro_rules! error {
    ($($arg:tt)*) => {
        $crate::log!($crate::log::LogLevel::Error, $($arg)*);
    };
}

#[macro_export]
macro_rules! fatal {
    ($($arg:tt)*) => {
        $crate::log!($crate::log::LogLevel::Fatal, $($arg)*);
    };
}

#[macro_export]
macro_rules! panic_log {
    ($($arg:tt)*) => {
        $crate::log!($crate::log::LogLevel::Panic, $($arg)*);
    };
}

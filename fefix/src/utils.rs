#![macro_use]

/// A handly macro for quick and dirty debugging. It reports the caller location
/// in the form of file plus line, and it also supports `format!` -like arguments.
macro_rules! dbglog {
    ($($arg:tt)*) => {{
        if std::cfg!(debug_assertions) {
            std::eprintln!("[{}:{}] {}", std::file!(), std::line!(), std::format!($($arg)*));
        }
    }}
}

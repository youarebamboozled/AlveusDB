#[macro_export]
macro_rules! info {
    ($($arg:tt)*) => ({
        if $crate::logger::log::log_level() <= $crate::logger::log_level::LogLevel::Info {
            println!("{} \x1B[32;1;1m[INFO]\x1B[0m\t{}", $crate::logger::log::date_and_time(), format_args!($($arg)*));
        }
    })
}

#[macro_export]
macro_rules! debug {
    ($($arg:tt)*) => ({
        if $crate::logger::log::log_level() <= $crate::logger::log_level::LogLevel::Debug {
            println!("{} \x1B[96;1;1m[DEBUG]\x1B[0m\t{}", $crate::logger::log::date_and_time(), format_args!($($arg)*));
        }
    })
}

#[macro_export]
macro_rules! verbose {
    ($($arg:tt)*) => ({
        if $crate::logger::log::log_level() <= $crate::logger::log_level::LogLevel::Verbose {
            println!("{} \x1B[35;1;1m[VERBOSE]\x1B[0m\t{}", $crate::logger::log::date_and_time(), format_args!($($arg)*));
        }
    })
}

#[macro_export]
macro_rules! warn {
    ($($arg:tt)*) => ({
        if $crate::logger::log::log_level() <= $crate::logger::log_level::LogLevel::Warn {
            println!("{} \x1B[33;1;1m[WARN]\x1B[0m\t{}", $crate::logger::log::date_and_time(), format_args!($($arg)*));
        }
    })
}

#[macro_export]
macro_rules! error {
    ($($arg:tt)*) => ({
        if $crate::logger::log::log_level() <= $crate::logger::log_level::LogLevel::Error {
            println!("{} \x1B[31;1;1m[ERROR]\x1B[0m\t{}", $crate::logger::log::date_and_time(), format_args!($($arg)*));
        }
    })
}

#[macro_export]
macro_rules! fatal {
    ($($arg:tt)*) => ({
        if $crate::logger::log::log_level() <= $crate::logger::log_level::LogLevel::Fatal {
            println!("{} \x1B[31;1;5;7m[FATAL]\x1B[0m\t{}", $crate::logger::log::date_and_time(), format_args!($($arg)*));
        }
    })
}
use crate::logger::log_level;
use chrono;

pub(crate) fn date_and_time() -> String {
    let now = std::time::SystemTime::now();
    let now = match now.duration_since(std::time::UNIX_EPOCH) {
        Ok(n) => n,
        Err(_) => panic!("SystemTime before UNIX EPOCH!"),
    };
    let now = now.as_secs();
    let now = chrono::NaiveDateTime::from_timestamp_opt(now as i64, 0);
    let now = chrono::DateTime::<chrono::Utc>::from_utc(match now {
        Some(n) => n,
        None => panic!("SystemTime before UNIX EPOCH!"),
    }, chrono::Utc);
    now.format("%Y-%m-%d %H:%M:%S").to_string()
}

pub(crate) fn log_level() -> log_level::LogLevel {
    let log_level = match std::env::var("ALVEUS_LOG") {
        Ok(log_level) => log_level,
        Err(_) => "info".to_string(),
    };
    match log_level::LogLevel::from_str(&log_level) {
        Ok(log_level) => log_level,
        Err(_) => log_level::LogLevel::Info,
    }
}
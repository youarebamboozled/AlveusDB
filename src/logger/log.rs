use crate::logger::log_level;
use chrono;

pub(crate) fn date_and_time() -> String {
    let now = std::time::SystemTime::now();
    let now = now.duration_since(std::time::UNIX_EPOCH).unwrap();
    let now = now.as_secs();
    let now = chrono::NaiveDateTime::from_timestamp_opt(now as i64, 0);
    let now = chrono::DateTime::<chrono::Utc>::from_utc(now.unwrap(), chrono::Utc);
    now.format("%Y-%m-%d %H:%M:%S").to_string()
}

pub(crate) fn log_level() -> log_level::LogLevel {
    let log_level = std::env::var("ALVEUS_LOG").unwrap_or("info".to_string());
    log_level::LogLevel::from_str(&log_level).unwrap()
}
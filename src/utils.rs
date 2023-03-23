
pub(crate) fn get_http_date() -> String {
    let now = std::time::SystemTime::now();
    let now = now.duration_since(std::time::UNIX_EPOCH).unwrap();
    let now = now.as_secs();
    let now = chrono::NaiveDateTime::from_timestamp_opt(now as i64, 0);
    let now = chrono::DateTime::<chrono::Utc>::from_utc(now.unwrap(), chrono::Utc);
    now.format("%a, %d %b %Y %H:%M:%S GMT").to_string()
}
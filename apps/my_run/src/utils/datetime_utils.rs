use chrono::{DateTime, Utc};
use chrono_tz::Tz;

pub fn get_time() -> String {
    format_local(Utc::now(), "Asia/Shanghai")
}

fn format_local(datetime: DateTime<Utc>, timezone: &str) -> String {
    let tz: Tz = timezone.parse().unwrap_or(chrono_tz::UTC);
    let local_time = datetime.with_timezone(&tz);
    local_time.format("%Y-%m-%d %H:%M:%S").to_string()
}
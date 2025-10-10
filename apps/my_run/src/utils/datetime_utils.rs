use chrono::{DateTime, Local};

pub async fn get_time() -> String {
    let now: DateTime<Local> = Local::now();
    now.format("%Y-%m-%d %H:%M:%S,%3f").to_string()
}
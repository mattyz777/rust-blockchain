# install

```bash
cargo add chrono chrono-tz
```

# utc vs local

```rs
use chrono::{DateTime, Local, Utc};

fn main() {
    let utc_now: DateTime<Utc> = Utc::now();
    let local_now: DateTime<Local> = Local::now();

    println!("UTC   : {}", utc_now);   // 2025-10-13 10:23:57.898098300 UTC
    println!("Local : {}", local_now); // 2025-10-13 18:23:57.898109600 +08:00
}
```

# format utc time to local string

```rs
use chrono::{DateTime, Utc};
use chrono_tz::Tz;

fn format_local(datetime: DateTime<Utc>, timezone: &str) -> String {
    let tz: Tz = timezone.parse().unwrap_or(chrono_tz::UTC);
    let local_time = datetime.with_timezone(&tz);
    local_time.format("%Y-%m-%d %H:%M:%S").to_string()
}

fn main() {
    let utc_time = Utc::now();
    println!("{}", format_local(utc_time, "Asia/Shanghai"));    // 2025-10-13 19:25:15
    println!("{}", format_local(utc_time, "America/New_York")); // 2025-10-13 06:25:15
}
```

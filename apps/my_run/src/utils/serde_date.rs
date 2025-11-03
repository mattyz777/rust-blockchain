use chrono::{DateTime, NaiveDate, TimeZone, Utc};
use serde::{self, Deserialize, Deserializer};

/// Deserialize an optional date string in format "YYYY-MM-DD" into `Option<DateTime<Utc>>`.
///
/// Example: "2024-12-12" → Some(DateTime<Utc> at midnight)

pub fn deserialize_option_yyyy_mm_dd<'de, D>(
    deserializer: D,
) -> Result<Option<DateTime<Utc>>, D::Error>
where
    D: Deserializer<'de>,
{
    let opt: Option<String> = Option::deserialize(deserializer)?;
    match opt {
        Some(s) => {
            let naive = NaiveDate::parse_from_str(&s, "%Y-%m-%d")
                .map_err(serde::de::Error::custom)?;
            Ok(Some(Utc.from_utc_datetime(
                &naive.and_hms_opt(0, 0, 0).unwrap(),
            )))
        }
        None => Ok(None),
    }
}

use chrono::{DateTime, NaiveDateTime, Utc};
use serde::{Deserialize, Deserializer};

/// Deserializes an optional `DateTime` from either ISO 8601 with timezone or naive format
/// Converts it to `NaiveDateTime` by extracting the UTC timestamp
pub fn deserialize_optional_naive_datetime<'de, D>(
    deserializer: D,
) -> Result<Option<NaiveDateTime>, D::Error>
where
    D: Deserializer<'de>,
{
    let opt_str: Option<String> = Option::deserialize(deserializer)?;

    match opt_str {
        None => Ok(None),
        Some(s) => {
            // Try parsing as DateTime<Utc> first (with timezone)
            if let Ok(dt) = s.parse::<DateTime<Utc>>() {
                return Ok(Some(dt.naive_utc()));
            }

            // Fall back to parsing as NaiveDateTime (without timezone)
            s.parse::<NaiveDateTime>()
                .map(Some)
                .map_err(serde::de::Error::custom)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::{Datelike, Timelike};
    use serde::Deserialize;

    #[derive(Deserialize)]
    struct TestStruct {
        #[serde(deserialize_with = "deserialize_optional_naive_datetime")]
        datetime: Option<NaiveDateTime>,
    }

    #[test]
    fn test_deserialize_with_timezone() {
        let json = r#"{"datetime": "2026-01-31T13:52:00.000Z"}"#;
        let result: TestStruct = serde_json::from_str(json).unwrap();
        assert!(result.datetime.is_some());
        let dt = result.datetime.unwrap();
        assert_eq!(dt.year(), 2026);
        assert_eq!(dt.month(), 1);
        assert_eq!(dt.day(), 31);
        assert_eq!(dt.hour(), 13);
        assert_eq!(dt.minute(), 52);
    }

    #[test]
    fn test_deserialize_without_timezone() {
        let json = r#"{"datetime": "2026-01-31T13:52:00"}"#;
        let result: TestStruct = serde_json::from_str(json).unwrap();
        assert!(result.datetime.is_some());
        let dt = result.datetime.unwrap();
        assert_eq!(dt.year(), 2026);
        assert_eq!(dt.month(), 1);
        assert_eq!(dt.day(), 31);
        assert_eq!(dt.hour(), 13);
        assert_eq!(dt.minute(), 52);
    }

    #[test]
    fn test_deserialize_none() {
        let json = r#"{"datetime": null}"#;
        let result: TestStruct = serde_json::from_str(json).unwrap();
        assert!(result.datetime.is_none());
    }
}

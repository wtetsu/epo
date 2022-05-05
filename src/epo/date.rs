use chrono::{DateTime, FixedOffset, Local, NaiveDate, NaiveDateTime, TimeZone, Utc};
use chrono_tz::Tz;
use once_cell::sync::Lazy;

pub struct EpochInfo {
    pub epoch_sec: i64,
    pub offset_sec: i32,
    pub date_str: String,
}

pub struct DateInfo {
    pub date_time: NaiveDateTime,
    pub date_str: String,
}

pub struct ParseSettings {
    pub date_formats_10: Vec<String>,
    pub date_formats_19: Vec<String>,
    pub date_formats_23: Vec<String>,
}

pub fn to_date_str(epoch_sec: i64, offset_sec: i32) -> String {
    let dt = Utc.timestamp(epoch_sec, 0).with_timezone(&FixedOffset::east(offset_sec));

    dt.format("%Y-%m-%dT%H:%M:%S%z").to_string()
}

pub fn to_date_str_with_tz(epoch_sec: i64, timezone: &str) -> String {
    let tz: Tz = timezone.parse().unwrap();
    let dt = tz.timestamp(epoch_sec, 0);
    dt.format("%Y-%m-%dT%H:%M:%S%z").to_string()
}

pub fn parse_date_with_offset_str(date_str: &str) -> Result<EpochInfo, String> {
    let formats = vec![
        "%Y-%m-%dT%H:%M:%S%z",
        "%Y/%m/%dT%H:%M:%S%z",
        "%Y-%m-%d %H:%M:%S%z",
        "%Y/%m/%d %H:%M:%S%z",
    ];

    for format in formats {
        if let Ok(dt) = DateTime::parse_from_str(date_str, format) {
            return Ok(EpochInfo {
                epoch_sec: dt.timestamp(),
                offset_sec: dt.offset().local_minus_utc(),
                date_str: date_str.to_string(),
            });
        }
    }

    if let Ok(dt) = DateTime::parse_from_rfc2822(date_str) {
        return Ok(EpochInfo {
            epoch_sec: dt.timestamp(),
            offset_sec: 0,
            date_str: date_str.to_string(),
        });
    }

    Err("Parse error".to_string())
}

pub fn parse_naive_date_str(date_str: &str, parse_config: &ParseSettings) -> Result<DateInfo, String> {
    let formats = match date_str.len() {
        10 => &parse_config.date_formats_10,
        19 => &parse_config.date_formats_19,
        23 => &parse_config.date_formats_23,
        _ => return Err("Parse error".to_string()),
    };

    for format in formats {
        if let Ok(date_time) = parse_date_str(date_str, format) {
            return Ok(date_time);
        }
    }

    Err("Parse error".to_string())
}

fn parse_date_str(date_str: &str, format: &str) -> Result<DateInfo, String> {
    if date_str.len() <= 10 {
        if let Ok(date) = NaiveDate::parse_from_str(date_str, format) {
            let date_time = date.and_hms(0, 0, 0);
            return Ok(DateInfo {
                date_time,
                date_str: date_str.to_string(),
            });
        }
    }

    if date_str.len() >= 19 {
        if let Ok(date_time) = NaiveDateTime::parse_from_str(date_str, format) {
            return Ok(DateInfo {
                date_time,
                date_str: date_str.to_string(),
            });
        }
    }

    Err("Parse error".to_string())
}

static START_DATE_TIME: Lazy<DateTime<Local>> = Lazy::new(Local::now);

pub fn current_epoch() -> i64 {
    (&START_DATE_TIME).timestamp()
}

pub fn current_date_info() -> EpochInfo {
    to_date_value((&START_DATE_TIME).with_timezone(&Local))
}

pub fn get_utc_offset_sec() -> i32 {
    (&START_DATE_TIME).date().offset().local_minus_utc()
}

fn to_date_value(time: DateTime<Local>) -> EpochInfo {
    let epoch_sec = time.timestamp();
    let offset_sec = time.date().offset().local_minus_utc();
    let date_str = to_date_str(epoch_sec, offset_sec / 3600);

    EpochInfo {
        epoch_sec,
        offset_sec,
        date_str,
    }
}

pub fn parse_offset_str(offset_str: &str) -> Result<i32, String> {
    if offset_str.len() == 5 {
        return parse_5letters_offset_str(offset_str);
    }

    parse_hours_offset_str(offset_str)
}

pub fn to_offset_str(offset_sec: i32) -> String {
    let sign = if offset_sec >= 0 { "+" } else { "-" };

    let hour = (offset_sec / 3600).abs();
    let min = (offset_sec % 3600).abs() / 60;

    format!("{}{:02}{:02}", sign, hour, min)
}

fn parse_5letters_offset_str(offset_str: &str) -> Result<i32, String> {
    if !(offset_str.starts_with('+') || offset_str.starts_with('-')) {
        return Err("Invalid offset".to_string());
    }
    let sign = if offset_str.starts_with('+') { 1 } else { -1 };

    if let Ok(h) = offset_str[1..3].parse::<i32>() {
        if let Ok(m) = offset_str[3..5].parse::<i32>() {
            let offset_sec = h * 3600 + m * 60;
            if offset_sec < 24 * 3600 {
                return Ok(offset_sec * sign);
            }
        }
    }
    Err("Invalid offset".to_string())
}

fn parse_hours_offset_str(offset_str: &str) -> Result<i32, String> {
    if !(offset_str.starts_with('+') || offset_str.starts_with('-')) {
        return Err("Invalid offset".to_string());
    }

    let sign = if offset_str.starts_with('+') { 1 } else { -1 };

    if let Ok(offset_hour) = offset_str[1..].parse::<i32>() {
        if offset_hour < 24 {
            return Ok(offset_hour * 3600 * sign);
        }
    }

    Err("Invalid offset".to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_to_date_str() {
        assert_eq!("1970-01-01T00:00:00+0000", to_date_str(0, 0));
        assert_eq!("1970-01-01T09:00:00+0900", to_date_str(0, 32400));
        assert_eq!("2022-04-17T21:09:49+0900", to_date_str(1650197389, 32400));
        assert_eq!("2022-04-17T12:09:49+0000", to_date_str(1650197389, 0));
        assert_eq!("2022-04-17T07:09:49-0500", to_date_str(1650197389, -18000));
    }

    #[test]
    fn test_parse_date_with_offset_str() {
        assert_date(0, 0, parse_date_with_offset_str("1970-01-01T00:00:00+0000").unwrap());
        assert_date(3600 * 9, 0, parse_date_with_offset_str("1970-01-01T09:00:00+0000").unwrap());
        assert_date(0, 32400, parse_date_with_offset_str("1970-01-01T09:00:00+0900").unwrap());
        assert_date(0, 0, parse_date_with_offset_str("1970/01/01T00:00:00+0000").unwrap());
        assert_date(32400, 0, parse_date_with_offset_str("1970/01/01T09:00:00+0000").unwrap());
        assert_date(0, 32400, parse_date_with_offset_str("1970/01/01T09:00:00+0900").unwrap());
    }

    #[test]
    fn test_to_date_str2() {
        assert_eq!("1970-01-01T00:00:00+0000", to_date_str_with_tz(0, "UTC"));
        assert_eq!("1970-01-01T00:00:00+0000", to_date_str_with_tz(0, "GMT"));
        assert_eq!("1970-01-01T09:00:00+0900", to_date_str_with_tz(0, "Asia/Tokyo"));

        assert_eq!("2022-04-17T21:09:49+0900", to_date_str_with_tz(1650197389, "Asia/Tokyo"));
        assert_eq!("2022-04-17T12:09:49+0000", to_date_str_with_tz(1650197389, "UTC"));
        assert_eq!("2022-04-17T08:09:49-0400", to_date_str_with_tz(1650197389, "America/New_York"));
        assert_eq!("2022-04-17T05:09:49-0700", to_date_str_with_tz(1650197389, "America/Phoenix"));

        assert_eq!("2022-01-01T02:30:40+0900", to_date_str_with_tz(1640971840, "Asia/Tokyo"));
        assert_eq!("2021-12-31T17:30:40+0000", to_date_str_with_tz(1640971840, "UTC"));
        assert_eq!("2021-12-31T12:30:40-0500", to_date_str_with_tz(1640971840, "America/New_York"));
        assert_eq!("2021-12-31T10:30:40-0700", to_date_str_with_tz(1640971840, "America/Phoenix"));
    }
    #[test]
    fn test_to_date_value() {
        let dt: Result<DateTime<Local>, _> = Local.datetime_from_str("2023/12/07 22:45:56", "%Y/%m/%d %H:%M:%S");
        assert!(to_date_value(dt.unwrap()).date_str.ends_with("+0000"));
    }

    fn assert_date(epoch_sec: i64, offset: i32, date_value: EpochInfo) {
        assert_eq!(epoch_sec, date_value.epoch_sec);
        assert_eq!(offset, date_value.offset_sec);
    }

    #[test]
    fn test_parse_5letters_offset_str() {
        assert_eq!(0, parse_offset_str("+0000").unwrap());
        assert_eq!(0, parse_offset_str("-0000").unwrap());
        assert_eq!(1800, parse_offset_str("+0030").unwrap());
        assert_eq!(-1800, parse_offset_str("-0030").unwrap());
        assert_eq!(3600 * 5, parse_offset_str("+0500").unwrap());
        assert_eq!(-(3600 * 5), parse_offset_str("-0500").unwrap());
        assert_eq!(3600 * 9, parse_offset_str("+0900").unwrap());
        assert_eq!(-(3600 * 9), parse_offset_str("-0900").unwrap());
        assert_eq!(3600 * 12 + 1800, parse_offset_str("+1230").unwrap());
        assert_eq!(-(3600 * 12 + 1800), parse_offset_str("-1230").unwrap());
        assert_eq!(3600 * 23, parse_offset_str("+2300").unwrap());
        assert_eq!(-(3600 * 23), parse_offset_str("-2300").unwrap());
        assert_eq!(3600 * 23 + 3540, parse_offset_str("+2359").unwrap());
        assert_eq!(-(3600 * 23 + 3540), parse_offset_str("-2359").unwrap());
        assert!(parse_offset_str("+2400").is_err());
        assert!(parse_offset_str("-2400").is_err());
        assert!(parse_offset_str("+00xx").is_err());
        assert!(parse_offset_str("-00xx").is_err());
        assert!(parse_offset_str("+xx00").is_err());
        assert!(parse_offset_str("-xx00").is_err());
    }

    #[test]
    fn test_parse_hours_offset_str() {
        assert_eq!(0, parse_hours_offset_str("+0").unwrap());
        assert_eq!(0, parse_hours_offset_str("-0").unwrap());
        assert_eq!(3600, parse_hours_offset_str("+1").unwrap());
        assert_eq!(-(3600), parse_hours_offset_str("-1").unwrap());
        assert_eq!(3600 * 9, parse_hours_offset_str("+9").unwrap());
        assert_eq!(-(3600 * 9), parse_hours_offset_str("-9").unwrap());
        assert_eq!(3600 * 15, parse_hours_offset_str("+15").unwrap());
        assert_eq!(-(3600 * 15), parse_hours_offset_str("-15").unwrap());
        assert_eq!(3600 * 23, parse_hours_offset_str("+23").unwrap());
        assert_eq!(-(3600 * 23), parse_hours_offset_str("-23").unwrap());
        assert!(parse_hours_offset_str("+24").is_err());
        assert!(parse_hours_offset_str("-24").is_err());
        assert!(parse_hours_offset_str("").is_err());
        assert!(parse_hours_offset_str("+").is_err());
        assert!(parse_hours_offset_str("-").is_err());
        assert!(parse_hours_offset_str("++").is_err());
        assert!(parse_hours_offset_str("--").is_err());
    }

    #[test]
    fn test_to_offset_str() {
        assert_eq!("+0000", to_offset_str(0));
        assert_eq!("+0030", to_offset_str(1800));
        assert_eq!("-0030", to_offset_str(-1800));
        assert_eq!("+0500", to_offset_str(3600 * 5));
        assert_eq!("-0500", to_offset_str(-3600 * 5));
        assert_eq!("+0900", to_offset_str(3600 * 9));
        assert_eq!("-0900", to_offset_str(-3600 * 9));
        assert_eq!("+1230", to_offset_str(3600 * 12 + 1800));
        assert_eq!("-1230", to_offset_str(-(3600 * 12 + 1800)));
        assert_eq!("+2300", to_offset_str(3600 * 23));
        assert_eq!("-2300", to_offset_str(-3600 * 23));
        assert_eq!("+2359", to_offset_str(3600 * 23 + 3540));
        assert_eq!("-2359", to_offset_str(-(3600 * 23 + 3540)));
    }
}

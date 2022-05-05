use chrono::{DateTime, FixedOffset, Local, NaiveDate, NaiveDateTime, TimeZone, Utc};
use chrono_tz::Tz;
use once_cell::sync::Lazy;

pub struct EpochInfo {
    pub epoch_sec: i64,
    pub offset_sec: i32,
    pub datestr: String,
}

pub struct DateInfo {
    pub date_time: NaiveDateTime,
    pub datestr: String,
}

pub struct ParseSettings {
    pub date_formats_10: Vec<String>,
    pub date_formats_16: Vec<String>,
    pub date_formats_19: Vec<String>,
    pub date_formats_23: Vec<String>,
}

pub fn to_datestr(epoch_sec: i64, offset_sec: i32) -> String {
    let dt = Utc.timestamp(epoch_sec, 0).with_timezone(&FixedOffset::east(offset_sec));

    dt.format("%Y-%m-%dT%H:%M:%S%z").to_string()
}

pub fn to_datestr_with_tz(epoch_sec: i64, timezone: &str) -> String {
    let tz: Tz = timezone.parse().unwrap();
    let dt = tz.timestamp(epoch_sec, 0);
    dt.format("%Y-%m-%dT%H:%M:%S%z").to_string()
}

pub fn parse_datestr_with_offset(datestr: &str, parse_settings: &ParseSettings) -> Result<EpochInfo, String> {
    if datestr.ends_with('Z') {
        if let Ok(epoch_info) = parse_naive_datestr(&datestr[0..datestr.len() - 1], parse_settings) {
            return Ok(EpochInfo {
                epoch_sec: epoch_info.date_time.timestamp(),
                offset_sec: 0,
                datestr: epoch_info.datestr,
            });
        }
        return Err("Parse error".to_string());
    }

    let formats = match datestr.len() {
        15 | 16 => &parse_settings.date_formats_10,
        21 | 22 => &parse_settings.date_formats_16,
        24 | 25 => &parse_settings.date_formats_19,
        28 | 29 => &parse_settings.date_formats_23,
        _ => return Err("Parse error".to_string()),
    };

    for format in formats {
        if let Ok(dt) = DateTime::parse_from_str(datestr, &format!("{}%z", format)) {
            return Ok(EpochInfo {
                epoch_sec: dt.timestamp(),
                offset_sec: dt.offset().local_minus_utc(),
                datestr: datestr.to_string(),
            });
        }
    }

    if let Ok(dt) = DateTime::parse_from_rfc2822(datestr) {
        return Ok(EpochInfo {
            epoch_sec: dt.timestamp(),
            offset_sec: 0,
            datestr: datestr.to_string(),
        });
    }

    Err("Parse error".to_string())
}

pub fn parse_naive_datestr(datestr: &str, parse_settings: &ParseSettings) -> Result<DateInfo, String> {
    let formats = match datestr.len() {
        10 => &parse_settings.date_formats_10,
        16 => &parse_settings.date_formats_16,
        19 => &parse_settings.date_formats_19,
        23 => &parse_settings.date_formats_23,
        _ => return Err("Parse error".to_string()),
    };

    for format in formats {
        if let Ok(date_time) = parse_datestr(datestr, format) {
            return Ok(date_time);
        }
    }
    Err("Parse error".to_string())
}

fn parse_datestr(datestr: &str, format: &str) -> Result<DateInfo, String> {
    if datestr.len() <= 10 {
        if let Ok(date) = NaiveDate::parse_from_str(datestr, format) {
            let date_time = date.and_hms(0, 0, 0);
            return Ok(DateInfo {
                date_time,
                datestr: datestr.to_string(),
            });
        }
    }

    if datestr.len() >= 16 {
        if let Ok(date_time) = NaiveDateTime::parse_from_str(datestr, format) {
            return Ok(DateInfo {
                date_time,
                datestr: datestr.to_string(),
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
    let datestr = to_datestr(epoch_sec, offset_sec / 3600);

    EpochInfo {
        epoch_sec,
        offset_sec,
        datestr,
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
    use super::super::arg::get_parse_settings;
    use super::*;

    #[test]
    fn test_to_datestr() {
        assert_eq!("1970-01-01T00:00:00+0000", to_datestr(0, 0));
        assert_eq!("1970-01-01T09:00:00+0900", to_datestr(0, 32400));
        assert_eq!("2022-04-17T21:09:49+0900", to_datestr(1650197389, 32400));
        assert_eq!("2022-04-17T12:09:49+0000", to_datestr(1650197389, 0));
        assert_eq!("2022-04-17T07:09:49-0500", to_datestr(1650197389, -18000));
    }

    #[test]
    fn test_parse_date_with_offset_str1() {
        let s = get_parse_settings();
        assert_date(0, 0, parse_datestr_with_offset("1970-01-01T00:00:00+0000", &s).unwrap());
        assert_date(3600 * 9, 0, parse_datestr_with_offset("1970-01-01T09:00:00+0000", &s).unwrap());
        assert_date(0, 32400, parse_datestr_with_offset("1970-01-01T09:00:00+0900", &s).unwrap());
        assert_date(0, 0, parse_datestr_with_offset("1970/01/01T00:00:00+0000", &s).unwrap());
        assert_date(32400, 0, parse_datestr_with_offset("1970/01/01T09:00:00+0000", &s).unwrap());
        assert_date(0, 32400, parse_datestr_with_offset("1970/01/01T09:00:00+0900", &s).unwrap());
    }

    #[test]
    fn test_parse_date_with_offset_str2() {
        let s = get_parse_settings();

        let datestrs = vec![
            "1970-01-01T00:00+0000",
            "1970-01-01T00:00+00:00",
            "1970-01-01T00:00:00+0000",
            "1970-01-01T00:00:00+00:00",
            "1970-01-01T09:00+0900",
            "1970-01-01T09:00+09:00",
            "1970-01-01T09:00:00+0900",
            "1970-01-01T09:00:00+09:00",
            "1970-01-01T05:00+0500",
            "1970-01-01T05:00+05:00",
            "1970-01-01T05:00:00+0500",
            "1970-01-01T05:00:00+05:00",
            "1969-12-31T23:00-0100",
            "1969-12-31T23:00-01:00",
            "1969-12-31T23:00:00-0100",
            "1969-12-31T23:00:00-01:00",
            "1969-12-31T20:00-0400",
            "1969-12-31T20:00-04:00",
            "1969-12-31T20:00:00-0400",
            "1969-12-31T20:00:00-04:00",
            "1969-12-31T10:00-1400",
            "1969-12-31T10:00-14:00",
            "1969-12-31T10:00:00-1400",
            "1969-12-31T10:00:00-14:00",
            "1970-01-01T00:00Z",
            "1970-01-01T00:00:00Z",
            "1970-01-01T00:00:00.000Z",
        ];

        for d in datestrs {
            assert_eq!(0, parse_datestr_with_offset(d, &s).unwrap().epoch_sec);
            assert_eq!(0, parse_datestr_with_offset(&d.replace('T', " "), &s).unwrap().epoch_sec);
        }
    }

    #[test]
    fn test_to_datestr2() {
        assert_eq!("1970-01-01T00:00:00+0000", to_datestr_with_tz(0, "UTC"));
        assert_eq!("1970-01-01T00:00:00+0000", to_datestr_with_tz(0, "GMT"));
        assert_eq!("1970-01-01T09:00:00+0900", to_datestr_with_tz(0, "Asia/Tokyo"));

        assert_eq!("2022-04-17T21:09:49+0900", to_datestr_with_tz(1650197389, "Asia/Tokyo"));
        assert_eq!("2022-04-17T12:09:49+0000", to_datestr_with_tz(1650197389, "UTC"));
        assert_eq!("2022-04-17T08:09:49-0400", to_datestr_with_tz(1650197389, "America/New_York"));
        assert_eq!("2022-04-17T05:09:49-0700", to_datestr_with_tz(1650197389, "America/Phoenix"));

        assert_eq!("2022-01-01T02:30:40+0900", to_datestr_with_tz(1640971840, "Asia/Tokyo"));
        assert_eq!("2021-12-31T17:30:40+0000", to_datestr_with_tz(1640971840, "UTC"));
        assert_eq!("2021-12-31T12:30:40-0500", to_datestr_with_tz(1640971840, "America/New_York"));
        assert_eq!("2021-12-31T10:30:40-0700", to_datestr_with_tz(1640971840, "America/Phoenix"));
    }
    #[test]
    fn test_to_date_value() {
        let dt: Result<DateTime<Local>, _> = Local.datetime_from_str("2023/12/07 22:45:56", "%Y/%m/%d %H:%M:%S");
        assert!(to_date_value(dt.unwrap()).datestr.ends_with("+0000"));
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

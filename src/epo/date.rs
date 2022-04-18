use chrono::{DateTime, FixedOffset, Local, TimeZone, Utc};

pub struct DateInfo {
    pub epoch_sec: i64,
    pub offset_sec: i32,
    pub date_str: String,
}

pub fn to_date_str(epoch_sec: i64, offset_sec: i32) -> String {
    let dt = Utc
        .timestamp(epoch_sec, 0)
        .with_timezone(&FixedOffset::east(offset_sec));

    return dt.format("%Y-%m-%dT%H:%M:%S%z").to_string();
}

pub fn parse_date_str(date_str: &str) -> Result<DateInfo, String> {
    let formats = vec![
        "%Y-%m-%dT%H:%M:%S%z",
        "%Y/%m/%dT%H:%M:%S%z",
        "%Y-%m-%d %H:%M:%S%z",
        "%Y/%m/%d %H:%M:%S%z",
    ];
    let formats_without_offset = vec![
        "%Y-%m-%dT%H:%M:%S",
        "%Y/%m/%dT%H:%M:%S",
        "%Y-%m-%d %H:%M:%S",
        "%Y/%m/%d %H:%M:%S",
    ];

    for format in formats {
        if let Ok(dt) = DateTime::parse_from_str(date_str, format) {
            return Ok(DateInfo {
                epoch_sec: dt.timestamp(),
                offset_sec: dt.offset().local_minus_utc(),
                date_str: date_str.to_string(),
            });
        }
    }

    for format in formats_without_offset {
        if let Ok(dt) = Utc.datetime_from_str(date_str, format) {
            return Ok(DateInfo {
                epoch_sec: dt.timestamp(),
                offset_sec: 0,
                date_str: date_str.to_string(),
            });
        }
    }

    if let Ok(dt) = DateTime::parse_from_rfc2822(date_str) {
        return Ok(DateInfo {
            epoch_sec: dt.timestamp(),
            offset_sec: 0,
            date_str: date_str.to_string(),
        });
    }

    Err("Parse error".to_string())
}

pub fn now() -> DateInfo {
    to_date_value(Local::now())
}

fn to_date_value(time: DateTime<Local>) -> DateInfo {
    let epoch_sec = time.timestamp();
    let offset_sec = time.date().offset().local_minus_utc();
    let date_str = to_date_str(epoch_sec, offset_sec / 3600);

    DateInfo {
        epoch_sec,
        offset_sec,
        date_str,
    }
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
    fn test_parse_date_str() {
        assert_date(0, 0, parse_date_str("1970-01-01T00:00:00+0000").unwrap());
        assert_date(3600 * 9, 0, parse_date_str("1970-01-01T09:00:00+0000").unwrap());
        assert_date(0, 32400, parse_date_str("1970-01-01T09:00:00+0900").unwrap());
        assert_date(0, 0, parse_date_str("1970/01/01T00:00:00+0000").unwrap());
        assert_date(32400, 0, parse_date_str("1970/01/01T09:00:00+0000").unwrap());
        assert_date(0, 32400, parse_date_str("1970/01/01T09:00:00+0900").unwrap());
        assert_date(0, 0, parse_date_str("1970/01/01 00:00:00").unwrap());
    }

    #[test]
    fn test_to_date_value() {
        let dt: Result<DateTime<Local>, _> = Local.datetime_from_str("2023/12/07 22:45:56", "%Y/%m/%d %H:%M:%S");
        assert!(to_date_value(dt.unwrap()).date_str.ends_with("+0000"));
    }

    fn assert_date(epoch_sec: i64, offset: i32, date_value: DateInfo) {
        assert_eq!(epoch_sec, date_value.epoch_sec);
        assert_eq!(offset, date_value.offset_sec);
    }
}

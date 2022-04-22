use super::{date, tz};
use chrono_tz::Tz;
use rhai::{Array, Dynamic, Engine};
use std::collections::HashSet;

pub struct Settings {
    pub dates: Vec<date::DateInfo>,
    pub timezones: Vec<TimeZone>,
}

pub enum TimeZone {
    Offset(i32),
    Tzname(String),
}

pub fn parse_arguments(args: Vec<String>) -> Result<Settings, Vec<String>> {
    if args.len() <= 1 {
        return Ok(make_default_settings());
    }

    let mut all_timezones: Vec<TimeZone> = Vec::new();
    let mut dates: Vec<date::DateInfo> = Vec::new();
    let mut errors: Vec<String> = Vec::new();

    for arg in args.iter().skip(1) {
        let r = parse_arg_value(arg);

        match r {
            ParseArgResult::UtcOffset(offset_secs) => all_timezones.push(TimeZone::Offset(offset_secs)),
            ParseArgResult::DateInfo(date_info) => {
                all_timezones.push(TimeZone::Offset(date_info.offset_sec));
                dates.push(date_info);
            }
            ParseArgResult::Tzname(tzname) => all_timezones.push(TimeZone::Tzname(tzname)),
            ParseArgResult::Epochs(epochs) => {
                let offset_sec = date::get_utc_offset_sec();
                for epoch_sec in epochs {
                    let date_str = arg.to_string();
                    dates.push(date::DateInfo {
                        epoch_sec,
                        offset_sec,
                        date_str,
                    });
                }
            }
            ParseArgResult::Error(error) => errors.push(error),
        }
    }

    if !errors.is_empty() {
        return Err(errors);
    }

    if all_timezones.is_empty() {
        all_timezones.push(TimeZone::Offset(date::current_date_info().offset_sec));
    }
    let timezones = unique(all_timezones);

    if dates.is_empty() {
        dates.push(date::current_date_info());
    }
    Ok(Settings { timezones, dates })
}

enum ParseArgResult {
    DateInfo(date::DateInfo),
    Epochs(Vec<i64>),
    UtcOffset(i32),
    Tzname(String),
    Error(String),
}

fn parse_arg_value(arg: &str) -> ParseArgResult {
    if arg.len() >= 2 && (arg.starts_with('+') || arg.starts_with('-')) {
        if let Ok(offset_sec) = date::parse_offset_str(arg) {
            return ParseArgResult::UtcOffset(offset_sec);
        }
    }

    // Date
    if let Ok(dt) = date::parse_date_str(arg) {
        return ParseArgResult::DateInfo(dt);
    }

    // Time zone name (exact match)
    if arg.parse::<Tz>().is_ok() {
        return ParseArgResult::Tzname(arg.to_string());
    }

    // Time zone name (search)
    let founds = tz::search(arg);
    if founds.len() == 1 {
        return ParseArgResult::Tzname(founds[0].to_string());
    }
    if founds.len() >= 2 {
        return ParseArgResult::Error(format!("Ambiguous timezone({})", founds.join(",")));
    }

    if let Ok(r) = eval(arg) {
        return ParseArgResult::Epochs(r);
    }

    ParseArgResult::Error(format!("Invalid value: {}", arg))
}

fn now() -> i64 {
    date::current_epoch()
}

fn eval(arg: &str) -> Result<Vec<i64>, String> {
    let mut engine = Engine::new();
    engine.register_fn("now", now);

    if let Ok(r) = engine.eval::<Dynamic>(arg) {
        if r.is::<i64>() {
            return Ok(vec![r.cast()]);
        }

        if r.is::<Array>() {
            let arr = r.cast::<Array>();

            let mut epochs: Vec<i64> = Vec::new();
            for a in arr {
                epochs.push(a.cast());
            }
            return Ok(epochs);
        }
    }

    return Err(format!("Invalid value: {}", arg));
}

fn unique(values: Vec<TimeZone>) -> Vec<TimeZone> {
    let mut int_set: HashSet<i32> = HashSet::new();
    let mut string_set: HashSet<String> = HashSet::new();

    let mut result: Vec<TimeZone> = Vec::new();
    for v in values {
        match v {
            TimeZone::Offset(offset_sec) => {
                if int_set.insert(offset_sec) {
                    result.push(TimeZone::Offset(offset_sec));
                }
            }
            TimeZone::Tzname(tzname) => {
                if string_set.insert(tzname.clone()) {
                    result.push(TimeZone::Tzname(tzname));
                }
            }
        }
    }
    result
}

fn make_default_settings() -> Settings {
    let now = date::current_date_info();
    Settings {
        timezones: vec![TimeZone::Offset(now.offset_sec)],
        dates: vec![now],
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_parse_arg_value_epoch() {
        let test_data: Vec<(&str, i32)> = vec![
            ("+1", 3600),
            ("-0", 0),
            ("+0", 0),
            ("+9", 3600 * 9),
            ("-5", -(3600 * 5)),
            ("+23", (3600 * 23)),
            ("-23", -(3600 * 23)),
        ];

        for (arg, expected) in test_data {
            let r = parse_arg_value(arg);
            match r {
                ParseArgResult::UtcOffset(offset) => assert_eq!(offset, expected),
                _ => unreachable!(),
            }
        }
    }

    #[test]
    fn test_parse_arg_value_date() {
        let test_data: Vec<(&str, i64)> = vec![
            ("1970-01-01T00:00:00+0000", 0),
            ("1970-01-01T09:00:00+0900", 0),
            ("2022-04-21T01:15:00+0900", 1650471300),
        ];

        for (arg, expected_epoch) in test_data {
            let r = parse_arg_value(arg);
            match r {
                ParseArgResult::DateInfo(date) => {
                    assert_eq!(expected_epoch, date.epoch_sec);
                }
                _ => unreachable!(),
            }
        }
    }

    #[test]
    fn test_parse_arg_value_error() {
        let test_data: Vec<&str> = vec!["+", "-", "x", "", "1x", "1.0.0"];

        for arg in test_data {
            let r = parse_arg_value(arg);

            if let ParseArgResult::Error(_) = r {
                continue;
            }
            unreachable!();
        }
    }
}

use super::{date, script, tz};
use chrono_tz::Tz;
use std::collections::HashSet;

pub struct Settings {
    pub epochs: Vec<date::EpochInfo>,
    pub dates: Vec<date::DateInfo>,
    pub timezones: Vec<TimeZone>,
    pub time_mode: TimeMode,
    pub print_mode: PrintMode,
    pub help: bool,
}

pub enum TimeZone {
    Offset(i32),
    Tzname(String),
}

pub enum TimeMode {
    Seconds,
    Milliseconds,
}

pub enum PrintMode {
    Markdown,
    PlainText,
}

pub fn parse_arguments(args: &[String]) -> Result<Settings, Vec<String>> {
    if args.len() <= 1 {
        return Ok(make_default_settings());
    }

    let mut all_timezones: Vec<TimeZone> = Vec::new();
    let mut epochs: Vec<date::EpochInfo> = Vec::new();
    let mut dates: Vec<date::DateInfo> = Vec::new();
    let mut errors: Vec<String> = Vec::new();
    let mut time_mode = TimeMode::Seconds;
    let mut print_mode = PrintMode::Markdown;
    let mut help = false;

    for arg in args.iter().skip(1) {
        match parse_arg_value(arg) {
            ParseArgResult::UtcOffset(offset_secs) => all_timezones.push(TimeZone::Offset(offset_secs)),
            ParseArgResult::EpochInfo(epoch_info) => {
                all_timezones.push(TimeZone::Offset(epoch_info.offset_sec));
                epochs.push(epoch_info);
            }
            ParseArgResult::Tzname(tzname) => all_timezones.push(TimeZone::Tzname(tzname)),
            ParseArgResult::Epochs(epoch) => {
                let offset_sec = date::get_utc_offset_sec();
                for epoch_sec in epoch {
                    let date_str = arg.to_string();
                    epochs.push(date::EpochInfo {
                        epoch_sec,
                        offset_sec,
                        date_str,
                    });
                }
            }
            ParseArgResult::DateInfo(date_info) => dates.push(date_info),
            ParseArgResult::TimeMode(new_time_mode) => time_mode = new_time_mode,
            ParseArgResult::PrintMode(new_print_mode) => print_mode = new_print_mode,
            ParseArgResult::Help(new_help) => help = new_help,
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

    if epochs.is_empty() && dates.is_empty() {
        epochs.push(date::current_date_info());
    }

    Ok(Settings {
        epochs,
        dates,
        timezones,
        time_mode,
        print_mode,
        help,
    })
}

enum ParseArgResult {
    EpochInfo(date::EpochInfo),
    Epochs(Vec<i64>),
    DateInfo(date::DateInfo),
    UtcOffset(i32),
    Tzname(String),
    Error(String),
    TimeMode(TimeMode),
    PrintMode(PrintMode),
    Help(bool),
}

fn parse_arg_value(arg: &str) -> ParseArgResult {
    if arg.len() >= 2 && (arg.starts_with('+') || arg.starts_with('-')) {
        if let Ok(offset_sec) = date::parse_offset_str(arg) {
            return ParseArgResult::UtcOffset(offset_sec);
        }
    }

    match arg {
        "-m" => return ParseArgResult::TimeMode(TimeMode::Milliseconds),
        "-p" => return ParseArgResult::PrintMode(PrintMode::PlainText),
        "-h" => return ParseArgResult::Help(true),
        _ => {}
    }

    // Date with offset
    if let Ok(dt) = date::parse_date_with_offset_str(arg) {
        return ParseArgResult::EpochInfo(dt);
    }

    // Date without offset
    if let Ok(dt) = date::parse_naive_date_str(arg) {
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

    match script::eval(arg) {
        Ok(r) => ParseArgResult::Epochs(r),
        Err(e) => ParseArgResult::Error(e),
    }
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
        epochs: vec![now],
        dates: vec![],
        time_mode: TimeMode::Seconds,
        print_mode: PrintMode::Markdown,
        help: false,
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
                ParseArgResult::EpochInfo(date) => {
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

    #[test]
    fn test_parse_arguments_default() {
        let actual = parse_arguments(&[]).unwrap();
        let now = date::current_date_info();

        assert_eq!(0, actual.dates.len());
        assert_eq!(1, actual.timezones.len());
        assert_eq!(1, actual.epochs.len());
        assert_eq!(now.epoch_sec, actual.epochs[0].epoch_sec);
    }

    #[test]
    fn test_parse_arguments() {
        let actual = parse_arguments(&[
            String::from("dummy"),
            String::from("2022-04-01"),
            String::from("0"),
            String::from("+9"),
            String::from("-5"),
            String::from("tokyo"),
            String::from("new_y"),
        ])
        .unwrap();

        assert_eq!(1, actual.dates.len());
        assert_eq!(4, actual.timezones.len());
        assert_eq!(1, actual.epochs.len());
    }
}

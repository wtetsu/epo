use super::types::{PrintMode, Settings, TimeMode, Zone};
use super::{date, script, tz};
use chrono_tz::Tz;
use std::collections::HashSet;

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

pub fn get_parse_settings() -> date::ParseSettings {
    let date_formats_10: Vec<String> = vec![
        "%Y-%m-%d".to_string(),
        "%Y/%m/%d".to_string(),
        //
    ];

    let date_formats_16: Vec<String> = vec![
        "%Y-%m-%dT%H:%M".to_string(),
        "%Y/%m/%dT%H:%M".to_string(),
        "%Y-%m-%d %H:%M".to_string(),
        "%Y/%m/%d %H:%M".to_string(),
    ];

    let date_formats_19: Vec<String> = vec![
        "%Y-%m-%dT%H:%M:%S".to_string(),
        "%Y/%m/%dT%H:%M:%S".to_string(),
        "%Y-%m-%d %H:%M:%S".to_string(),
        "%Y/%m/%d %H:%M:%S".to_string(),
    ];
    let date_formats_23: Vec<String> = vec![
        "%Y-%m-%dT%H:%M:%S.%3f".to_string(),
        "%Y/%m/%dT%H:%M:%S.%3f".to_string(),
        "%Y-%m-%d %H:%M:%S.%3f".to_string(),
        "%Y/%m/%d %H:%M:%S.%3f".to_string(),
        "%Y-%m-%dT%H:%M:%S,%3f".to_string(),
        "%Y/%m/%dT%H:%M:%S,%3f".to_string(),
        "%Y-%m-%d %H:%M:%S,%3f".to_string(),
        "%Y/%m/%d %H:%M:%S,%3f".to_string(),
    ];

    date::ParseSettings {
        date_formats_10,
        date_formats_16,
        date_formats_19,
        date_formats_23,
    }
}

pub fn parse_arguments(args: &[String], parse_settings: &date::ParseSettings) -> Result<Settings, Vec<String>> {
    if args.len() <= 1 {
        return Ok(make_default_settings());
    }

    let mut all_timezones: Vec<Zone> = Vec::new();
    let mut epochs: Vec<date::EpochInfo> = Vec::new();
    let mut dates: Vec<date::DateInfo> = Vec::new();
    let mut errors: Vec<String> = Vec::new();
    let mut time_mode = TimeMode::Seconds;
    let mut print_mode = PrintMode::Markdown;
    let mut help = false;

    for arg in args.iter().skip(1) {
        match parse_arg_value(arg, parse_settings) {
            ParseArgResult::UtcOffset(offset_secs) => all_timezones.push(Zone::Offset(offset_secs)),
            ParseArgResult::EpochInfo(epoch_info) => {
                all_timezones.push(Zone::Offset(epoch_info.offset_sec));
                epochs.push(epoch_info);
            }
            ParseArgResult::Tzname(tzname) => all_timezones.push(Zone::Tzname(tzname)),
            ParseArgResult::Epochs(epoch) => {
                let offset_sec = date::get_utc_offset_sec();
                for epoch_sec in epoch {
                    let datestr = arg.to_string();
                    epochs.push(date::EpochInfo {
                        epoch_sec,
                        offset_sec,
                        datestr,
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
        all_timezones.push(Zone::Offset(date::current_date_info().offset_sec));
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

fn parse_arg_value(arg: &str, parse_settings: &date::ParseSettings) -> ParseArgResult {
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
    if let Ok(dt) = date::parse_datestr_with_offset(arg, parse_settings) {
        return ParseArgResult::EpochInfo(dt);
    }

    // Date without offset
    if let Ok(dt) = date::parse_naive_datestr(arg, parse_settings) {
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

fn unique(values: Vec<Zone>) -> Vec<Zone> {
    let mut int_set: HashSet<i32> = HashSet::new();
    let mut string_set: HashSet<String> = HashSet::new();

    let mut result: Vec<Zone> = Vec::new();
    for v in values {
        match v {
            Zone::Offset(offset_sec) => {
                if int_set.insert(offset_sec) {
                    result.push(Zone::Offset(offset_sec));
                }
            }
            Zone::Tzname(tzname) => {
                if string_set.insert(tzname.clone()) {
                    result.push(Zone::Tzname(tzname));
                }
            }
        }
    }
    result
}

fn make_default_settings() -> Settings {
    let now = date::current_date_info();
    Settings {
        timezones: vec![Zone::Offset(now.offset_sec)],
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
            let r = parse_arg_value(arg, &get_parse_settings());
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
            let r = parse_arg_value(arg, &get_parse_settings());
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
            let r = parse_arg_value(arg, &get_parse_settings());

            if let ParseArgResult::Error(_) = r {
                continue;
            }
            unreachable!();
        }
    }

    #[test]
    fn test_parse_arguments_default() {
        let actual = parse_arguments(&[], &get_parse_settings()).unwrap();
        let now = date::current_date_info();

        assert_eq!(0, actual.dates.len());
        assert_eq!(1, actual.timezones.len());
        assert_eq!(1, actual.epochs.len());
        assert_eq!(now.epoch_sec, actual.epochs[0].epoch_sec);
    }

    #[test]
    fn test_parse_arguments() {
        let actual = parse_arguments(
            &[
                "dummy".to_string(),
                "2022-04-01".to_string(),
                "2022-04-01T12:23".to_string(),
                "2022-04-01T12:23:34".to_string(),
                "2022-04-01T12:23:34.567".to_string(),
                "2022-04-01T12:23+0000".to_string(),
                "2022-04-01T12:23:34+0900".to_string(),
                "0".to_string(),
                "[1651789910,1651789910+86400]".to_string(),
                "+9".to_string(),
                "-5".to_string(),
                "tokyo".to_string(),
                "new_y".to_string(),
                "America/Los_Angeles".to_string(),
            ],
            &get_parse_settings(),
        )
        .unwrap();

        assert_eq!(4, actual.dates.len());
        assert_eq!(6, actual.timezones.len());
        assert_eq!(5, actual.epochs.len());
    }

    #[test]
    fn test_parse_arguments_global_options() {
        let actual = parse_arguments(
            &[
                "dummy".to_string(),
                "-h".to_string(),
                "-p".to_string(),
                //
            ],
            &get_parse_settings(),
        )
        .unwrap();

        match actual.print_mode {
            PrintMode::PlainText => (),
            _ => unreachable!(),
        }

        assert!(actual.help);
        assert_eq!(0, actual.dates.len());
        assert_eq!(1, actual.timezones.len());
        assert_eq!(1, actual.epochs.len());
    }
}

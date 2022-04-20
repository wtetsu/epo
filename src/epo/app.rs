use super::{date, print, util};
use std::collections::HashSet;

pub struct Settings {
    pub offset_secs: Vec<i32>,
    pub dates: Vec<date::DateInfo>,
}

pub fn run(settings: Settings) {
    if !settings.dates.is_empty() {
        let data = to_string_rows(settings.dates, settings.offset_secs);
        print::print_markdown_table(&data);
    }
}

pub fn parse_arguments(args: Vec<String>) -> Result<Settings, Vec<String>> {
    if args.len() <= 1 {
        return Ok(make_default_settings());
    }
    let mut all_offset_secs: Vec<i32> = Vec::new();
    let mut dates: Vec<date::DateInfo> = Vec::new();
    let mut errors: Vec<String> = Vec::new();

    for arg in args.iter().skip(1) {
        let r = parse_arg_value(arg);

        match r {
            ParseArgResult::Offset(offset_secs) => all_offset_secs.push(offset_secs),
            ParseArgResult::DateInfo(date_info) => {
                all_offset_secs.push(date_info.offset_sec);
                dates.push(date_info);
            }
            ParseArgResult::Error(error) => errors.push(error),
        }
    }

    if !errors.is_empty() {
        return Err(errors);
    }

    if all_offset_secs.is_empty() {
        all_offset_secs.push(date::now().offset_sec);
    }
    let offset_secs = unique(all_offset_secs);

    if dates.is_empty() {
        dates.push(date::now());
    }
    Ok(Settings { offset_secs, dates })
}

enum ParseArgResult<T1, T2, T3> {
    Offset(T1),
    DateInfo(T2),
    Error(T3),
}

fn parse_arg_value(arg: &str) -> ParseArgResult<i32, date::DateInfo, String> {
    if arg.len() >= 2 && (arg.starts_with('+') || arg.starts_with('-')) {
        if let Ok(offset_sec) = date::parse_offset_str(arg) {
            return ParseArgResult::Offset(offset_sec);
        }
    }
    if util::is_numeric(arg) {
        let epoch_sec: i64 = arg.parse().unwrap();
        let offset_sec = date::get_utc_offset_sec();
        let date_str = arg.to_string();

        return ParseArgResult::DateInfo(date::DateInfo {
            epoch_sec,
            offset_sec,
            date_str,
        });
    }
    if let Ok(dt) = date::parse_date_str(arg) {
        return ParseArgResult::DateInfo(dt);
    }

    ParseArgResult::Error(format!("Invalid value: {}", arg))
}

fn unique(values: Vec<i32>) -> Vec<i32> {
    let mut uniq: HashSet<i32> = HashSet::new();

    let mut result: Vec<i32> = Vec::new();
    for v in values {
        if uniq.insert(v) {
            result.push(v);
        }
    }
    result
}

fn make_default_settings() -> Settings {
    let now = date::now();
    Settings {
        offset_secs: vec![now.offset_sec],
        dates: vec![now],
    }
}

fn to_string_rows(dates: Vec<date::DateInfo>, offset_secs: Vec<i32>) -> Vec<Vec<String>> {
    let mut headers: Vec<String> = vec!["Epoch".to_string()];
    for offset_sec in &offset_secs {
        headers.push(date::to_offset_str(*offset_sec));
    }

    let mut rows: Vec<Vec<String>> = vec![headers];

    for date in &dates {
        let mut row: Vec<String> = vec![date.epoch_sec.to_string()];
        for offset_sec in &offset_secs {
            let dt = date::to_date_str(date.epoch_sec, *offset_sec);
            row.push(dt);
        }
        rows.push(row);
    }
    rows
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
                ParseArgResult::Offset(offset) => assert_eq!(offset, expected),
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
        let test_data: Vec<&str> = vec!["+", "-", "x", "", "1x", "1.0"];

        for arg in test_data {
            let r = parse_arg_value(arg);

            if let ParseArgResult::Error(_) = r {
                continue;
            }
            unreachable!();
        }
    }
}

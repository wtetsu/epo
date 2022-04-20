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

pub fn parse_arguments(args: Vec<String>) -> Settings {
    if args.len() <= 1 {
        return make_default_settings();
    }
    let mut all_offset_secs: Vec<i32> = Vec::new();
    let mut dates: Vec<date::DateInfo> = Vec::new();
    for arg in args.iter().skip(1) {
        if arg.len() >= 2 && (arg.starts_with('+') || arg.starts_with('-')) {
            if let Ok(offset_sec) = date::parse_offset_str(arg) {
                all_offset_secs.push(offset_sec);
            }
        } else if util::is_numeric(arg) {
            let epoch_sec: i64 = arg.parse().unwrap();
            let offset_sec = date::get_utc_offset_sec();
            let date_str = arg.to_string();
            dates.push(date::DateInfo {
                epoch_sec,
                offset_sec,
                date_str,
            });
        } else if let Ok(dt) = date::parse_date_str(arg) {
            all_offset_secs.push(dt.offset_sec);
            dates.push(dt);
        } else {
            eprintln!("Invalid date: {}", arg);
        }
    }
    if all_offset_secs.is_empty() {
        all_offset_secs.push(date::now().offset_sec);
    }
    let offset_secs = unique(all_offset_secs);

    if dates.is_empty() {
        dates.push(date::now());
    }
    Settings { offset_secs, dates }
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

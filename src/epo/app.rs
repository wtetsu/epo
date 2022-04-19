use std::collections::HashSet;

pub struct Settings {
    pub offset_secs: Vec<i32>,
    pub dates: Vec<super::date::DateInfo>,
}

pub fn parse_arguments(args: Vec<String>) -> Settings {
    if args.len() <= 1 {
        return make_default_settings();
    }
    let mut all_offset_secs: Vec<i32> = Vec::new();
    let mut dates: Vec<super::date::DateInfo> = Vec::new();
    for arg in args.iter().skip(1) {
        if arg.len() >= 2 && (arg.starts_with('+') || arg.starts_with('-')) {
            if let Ok(offset_sec) = super::date::parse_offset_str(arg) {
                all_offset_secs.push(offset_sec);
            }
        } else if super::util::is_numeric(arg) {
            let epoch_sec: i64 = arg.parse().unwrap();
            let offset_sec = super::date::get_utc_offset_sec();
            let date_str = arg.to_string();
            dates.push(super::date::DateInfo {
                epoch_sec,
                offset_sec,
                date_str,
            });
        } else if let Ok(dt) = super::date::parse_date_str(arg) {
            all_offset_secs.push(dt.offset_sec);
            dates.push(dt);
        } else {
            eprintln!("Invalid date: {}", arg);
        }
    }
    if all_offset_secs.is_empty() {
        all_offset_secs.push(super::date::now().offset_sec);
    }
    let offset_secs = unique(all_offset_secs);
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

pub fn make_default_settings() -> Settings {
    let now = super::date::now();
    Settings {
        offset_secs: vec![now.offset_sec],
        dates: vec![now],
    }
}

pub fn run(settings: Settings) {
    if !settings.dates.is_empty() {
        print_dates(settings.dates, settings.offset_secs)
    }
}

fn print_dates(dates: Vec<super::date::DateInfo>, offset_secs: Vec<i32>) {
    for d in dates {
        let date_strings = offset_secs
            .iter()
            .map(|o| super::date::to_date_str(d.epoch_sec, *o))
            .collect::<Vec<_>>();
        print_date_strings(d.epoch_sec, date_strings);
    }
}

fn print_date_strings(epoch: i64, date_strings: Vec<String>) {
    print!("{:10} ", epoch);
    for (i, d) in date_strings.iter().enumerate() {
        if i > 0 {
            print!(" ");
        }
        print!("{}", d);
    }
    println!();
}

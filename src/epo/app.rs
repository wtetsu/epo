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

    if dates.is_empty() {
        dates.push(super::date::now());
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

pub fn make_default_settings() -> Settings {
    let now = super::date::now();
    Settings {
        offset_secs: vec![now.offset_sec],
        dates: vec![now],
    }
}

pub fn run(settings: Settings) {
    if !settings.dates.is_empty() {
        // print_dates(settings.dates, settings.offset_secs);
        let data = to_string_rows(settings.dates, settings.offset_secs);
        print_markdown_table(&data);
    }
}

fn print_markdown_table(data: &Vec<Vec<String>>) {
    let max_lengths = calc_max_column_length(data);

    for (i, row) in data.iter().enumerate() {
        let line: Vec<String> = Vec::new();
        for (i, cell) in row.iter().enumerate() {
            let width = max_lengths[i];
            print!("| {:>width$} ", cell);
        }
        print!("|");
        println!();

        if i == 0 {
            println!("{}", generate_header_line(&max_lengths));
        }
    }
}

fn generate_header_line(max_lengths: &Vec<usize>) -> String {
    let mut header_line = "".to_string();
    for max_length in max_lengths.iter() {
        header_line.push_str("| ");
        header_line.push_str("-".repeat(*max_length).as_str());
        header_line.push_str(" ");
    }
    header_line.push_str("|");
    header_line
}

fn calc_max_column_length(data: &Vec<Vec<String>>) -> Vec<usize> {
    let mut max_len: Vec<usize> = vec![0; data[0].len()];

    for row in data {
        for (i, col) in row.iter().enumerate() {
            if max_len[i] < col.len() {
                max_len[i] = col.len();
            }
        }
    }

    max_len
}

fn to_string_rows(dates: Vec<super::date::DateInfo>, offset_secs: Vec<i32>) -> Vec<Vec<String>> {
    let mut headers: Vec<String> = vec!["Epoch".to_string()];
    for offset_sec in &offset_secs {
        headers.push(super::date::to_offset_str(*offset_sec));
    }

    let mut rows: Vec<Vec<String>> = vec![headers];

    for date in &dates {
        let mut row: Vec<String> = vec![date.epoch_sec.to_string()];
        for offset_sec in &offset_secs {
            let dt = super::date::to_date_str(date.epoch_sec, *offset_sec);
            row.push(dt);
        }
        rows.push(row);
    }
    rows
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

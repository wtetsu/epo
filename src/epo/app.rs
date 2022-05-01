use super::{arg, print};

pub fn run(settings: &arg::Settings) {
    if !settings.epochs.is_empty() {
        let (header, data) = print::to_string_rows_from_epochs(&settings.epochs, &settings.timezones);
        print(&header, &data, &settings.print_mode);
    }

    if !settings.dates.is_empty() {
        if !settings.epochs.is_empty() {
            println!();
        }
        let (header, data) = print::to_string_rows_from_dates(&settings.dates, &settings.timezones);
        print(&header, &data, &settings.print_mode);
    }
}

pub fn print(header: &[String], data: &[Vec<String>], mode: &arg::PrintMode) {
    match mode {
        arg::PrintMode::Markdown => print::print_markdown_table(header, data),
        arg::PrintMode::PlainText => print::print_as_plaintext(data, " "),
    }
}

#[cfg(test)]
mod tests {
    use super::super::date;
    use super::*;

    #[test]
    fn test_run_empty() {
        let settings = arg::Settings {
            timezones: vec![],
            epochs: vec![],
            dates: vec![],
            time_mode: arg::TimeMode::Seconds,
            print_mode: arg::PrintMode::Markdown,
        };
        run(&settings);
    }

    #[test]
    fn test_run_epochs() {
        let settings = arg::Settings {
            timezones: vec![
                arg::TimeZone::Offset(3600 * 9),
                arg::TimeZone::Offset(0),
                arg::TimeZone::Offset(-3600 * 5),
                //
            ],
            epochs: vec![
                date::EpochInfo {
                    epoch_sec: 0,
                    offset_sec: 0,
                    date_str: "".to_string(),
                },
                date::EpochInfo {
                    epoch_sec: 1651306548,
                    offset_sec: 0,
                    date_str: "".to_string(),
                },
            ],
            dates: vec![],
            time_mode: arg::TimeMode::Seconds,
            print_mode: arg::PrintMode::Markdown,
        };
        run(&settings);
    }
}

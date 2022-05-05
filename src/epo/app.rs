use super::types::{PrintMode, Settings};
use super::{date, help, print};

pub fn run(settings: &Settings) {
    if settings.help {
        println!("{}", help::usage(date::current_epoch()));
        return;
    }
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

pub fn print(header: &[String], data: &[Vec<String>], mode: &PrintMode) {
    match mode {
        PrintMode::Markdown => print::print_markdown_table(header, data),
        PrintMode::PlainText => print::print_as_plaintext(data, " "),
    }
}

#[cfg(test)]
mod tests {
    use super::super::date;
    use super::super::types::{PrintMode, Settings, TimeMode, Zone};
    use super::*;

    #[test]
    fn test_run_empty() {
        let settings = Settings {
            timezones: vec![],
            epochs: vec![],
            dates: vec![],
            time_mode: TimeMode::Seconds,
            print_mode: PrintMode::Markdown,
            help: false,
        };
        run(&settings);
    }

    #[test]
    fn test_run_epochs_markdown() {
        let settings = Settings {
            timezones: vec![
                Zone::Offset(3600 * 9),
                Zone::Offset(0),
                Zone::Offset(-3600 * 5), //
            ],
            epochs: vec![
                date::EpochInfo {
                    epoch_sec: 0,
                    offset_sec: 0,
                    datestr: "".to_string(),
                },
                date::EpochInfo {
                    epoch_sec: 1651306548,
                    offset_sec: 0,
                    datestr: "".to_string(),
                },
            ],
            dates: vec![],
            time_mode: TimeMode::Seconds,
            print_mode: PrintMode::Markdown,
            help: false,
        };
        run(&settings);
    }

    #[test]
    fn test_run_epochs_plaintext() {
        let settings = Settings {
            timezones: vec![
                Zone::Offset(3600 * 9),
                Zone::Offset(0),
                Zone::Offset(-3600 * 5),
                //
            ],
            epochs: vec![
                date::EpochInfo {
                    epoch_sec: 0,
                    offset_sec: 0,
                    datestr: "".to_string(),
                },
                date::EpochInfo {
                    epoch_sec: 1651306548,
                    offset_sec: 0,
                    datestr: "".to_string(),
                },
            ],
            dates: vec![],
            time_mode: TimeMode::Seconds,
            print_mode: PrintMode::PlainText,
            help: false,
        };
        run(&settings);
    }

    #[test]
    fn test_run_help() {
        let settings = Settings {
            timezones: vec![],
            epochs: vec![],
            dates: vec![],
            time_mode: TimeMode::Seconds,
            print_mode: PrintMode::Markdown,
            help: true,
        };
        run(&settings);
    }
}

use super::{arg, date, print};
use chrono::{FixedOffset, TimeZone};
use chrono_tz::Tz;

pub fn run(settings: arg::Settings) {
    if !settings.epochs.is_empty() {
        let data = to_string_rows_from_epochs(&settings.epochs, &settings.timezones);
        print::print_markdown_table(&data);
    }

    if !settings.dates.is_empty() {
        if !settings.epochs.is_empty() {
            println!();
        }
        let data = to_string_rows_from_dates(&settings.dates, &settings.timezones);
        print::print_markdown_table(&data);
    }
}

fn to_string_rows_from_epochs(epoch_infos: &Vec<date::EpochInfo>, timezones: &Vec<arg::TimeZone>) -> Vec<Vec<String>> {
    let mut headers: Vec<String> = vec!["Epoch".to_string()];
    for t in timezones {
        match t {
            arg::TimeZone::Offset(offset_sec) => headers.push(date::to_offset_str(*offset_sec)),
            arg::TimeZone::Tzname(tzname) => headers.push(tzname.to_string()),
        }
    }

    let mut rows: Vec<Vec<String>> = vec![headers];

    for date in epoch_infos {
        let mut row: Vec<String> = vec![date.epoch_sec.to_string()];
        for t in timezones {
            match t {
                arg::TimeZone::Offset(offset_sec) => row.push(date::to_date_str(date.epoch_sec, *offset_sec)),
                arg::TimeZone::Tzname(tzname) => row.push(date::to_date_str_with_tz(date.epoch_sec, tzname)),
            }
        }
        rows.push(row);
    }
    rows
}

fn to_string_rows_from_dates(date_infos: &Vec<date::DateInfo>, timezones: &Vec<arg::TimeZone>) -> Vec<Vec<String>> {
    let mut headers: Vec<String> = vec!["Date".to_string()];
    for t in timezones {
        match t {
            arg::TimeZone::Offset(offset_sec) => headers.push(date::to_offset_str(*offset_sec)),
            arg::TimeZone::Tzname(tzname) => headers.push(tzname.to_string()),
        }
    }
    let mut rows: Vec<Vec<String>> = vec![headers];

    for date in date_infos {
        let mut row: Vec<String> = vec![date.date_str.to_string()];
        for t in timezones {
            match t {
                arg::TimeZone::Offset(offset_sec) => {
                    let dt = FixedOffset::east(*offset_sec).from_local_datetime(&date.date_time).unwrap();
                    row.push(dt.timestamp().to_string());
                }
                arg::TimeZone::Tzname(tzname) => {
                    let tz: Tz = tzname.parse().unwrap();
                    let dt = tz.from_local_datetime(&date.date_time).unwrap();
                    row.push(dt.timestamp().to_string());
                }
            }
        }
        rows.push(row);
    }

    rows
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_to_string_rows_empty() {
        let epochs: Vec<date::EpochInfo> = Vec::new();
        let timezones: Vec<arg::TimeZone> = Vec::new();

        let r = to_string_rows_from_epochs(&epochs, &timezones);
        assert_eq!(1, r.len());
    }
}

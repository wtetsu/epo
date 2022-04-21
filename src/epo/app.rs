use super::{arg, date, print};

pub fn run(settings: arg::Settings) {
    if !settings.dates.is_empty() {
        let data = to_string_rows(settings.dates, settings.timezones);
        print::print_markdown_table(&data);
    }
}

fn to_string_rows(dates: Vec<date::DateInfo>, timezones: Vec<arg::TimeZone>) -> Vec<Vec<String>> {
    let mut headers: Vec<String> = vec!["Epoch".to_string()];
    for t in &timezones {
        match t {
            arg::TimeZone::Offset(offset_sec) => headers.push(date::to_offset_str(*offset_sec)),
            arg::TimeZone::Tzname(tzname) => headers.push(tzname.to_string()),
        }
    }

    let mut rows: Vec<Vec<String>> = vec![headers];

    for date in &dates {
        let mut row: Vec<String> = vec![date.epoch_sec.to_string()];
        for t in &timezones {
            match t {
                arg::TimeZone::Offset(offset_sec) => row.push(date::to_date_str(date.epoch_sec, *offset_sec)),
                arg::TimeZone::Tzname(tzname) => row.push(date::to_date_str_with_tz(date.epoch_sec, tzname)),
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
        let dates: Vec<date::DateInfo> = Vec::new();
        let timezones: Vec<arg::TimeZone> = Vec::new();

        let r = to_string_rows(dates, timezones);
        assert_eq!(1, r.len());
    }
}

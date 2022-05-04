use super::{arg, date};
use chrono::{FixedOffset, TimeZone};
use chrono_tz::Tz;
use std::io::{stdout, BufWriter, Write};

pub fn to_string_rows_from_epochs(epoch_infos: &Vec<date::EpochInfo>, timezones: &Vec<arg::Zone>) -> (Vec<String>, Vec<Vec<String>>) {
    let mut headers: Vec<String> = vec!["Epoch".to_string()];
    for t in timezones {
        match t {
            arg::Zone::Offset(offset_sec) => headers.push(date::to_offset_str(*offset_sec)),
            arg::Zone::Tzname(tzname) => headers.push(tzname.to_string()),
        }
    }

    let mut rows: Vec<Vec<String>> = vec![];

    for date in epoch_infos {
        let mut row: Vec<String> = vec![date.epoch_sec.to_string()];
        for t in timezones {
            let s = match t {
                arg::Zone::Offset(offset_sec) => date::to_date_str(date.epoch_sec, *offset_sec),
                arg::Zone::Tzname(tzname) => date::to_date_str_with_tz(date.epoch_sec, tzname),
            };
            row.push(s);
        }
        rows.push(row);
    }
    (headers, rows)
}

pub fn to_string_rows_from_dates(date_infos: &Vec<date::DateInfo>, timezones: &Vec<arg::Zone>) -> (Vec<String>, Vec<Vec<String>>) {
    let mut headers: Vec<String> = vec!["Date".to_string()];
    for t in timezones {
        match t {
            arg::Zone::Offset(offset_sec) => headers.push(date::to_offset_str(*offset_sec)),
            arg::Zone::Tzname(tzname) => headers.push(tzname.to_string()),
        }
    }
    let mut rows: Vec<Vec<String>> = vec![];

    for date in date_infos {
        let mut row: Vec<String> = vec![date.date_str.to_string()];
        for t in timezones {
            match t {
                arg::Zone::Offset(offset_sec) => {
                    let dt = FixedOffset::east(*offset_sec).from_local_datetime(&date.date_time).unwrap();
                    row.push(dt.timestamp().to_string());
                }
                arg::Zone::Tzname(tzname) => {
                    let tz: Tz = tzname.parse().unwrap();
                    let dt = tz.from_local_datetime(&date.date_time).unwrap();
                    row.push(dt.timestamp().to_string());
                }
            }
        }
        rows.push(row);
    }

    (headers, rows)
}

#[allow(unused_must_use)]
pub fn print_markdown_table(header: &[String], data: &[Vec<String>]) {
    let out = stdout();
    let mut buf = BufWriter::new(out.lock());

    let max_lengths = calc_max_column_length(header, data);

    for (i, cell) in header.iter().enumerate() {
        let width = max_lengths[i];
        write!(buf, "| {:>width$} ", cell);
    }
    writeln!(buf, "|");
    writeln!(buf, "{}", generate_header_line(&max_lengths));
    for (_, row) in data.iter().enumerate() {
        for (i, cell) in row.iter().enumerate() {
            let width = max_lengths[i];
            write!(buf, "| {:>width$} ", cell);
        }
        writeln!(buf, "|");
    }
}

#[allow(unused_must_use)]
pub fn print_as_plaintext(data: &[Vec<String>], delimiter: &str) {
    let out = stdout();
    let mut buf = BufWriter::new(out.lock());

    for (_, row) in data.iter().enumerate() {
        for (i, cell) in row.iter().enumerate() {
            if i > 0 {
                write!(buf, "{}", delimiter);
            }
            write!(buf, "{}", cell);
        }
        writeln!(buf);
    }
}

fn generate_header_line(max_lengths: &[usize]) -> String {
    let mut header_line = "".to_string();
    for max_length in max_lengths.iter() {
        header_line.push_str("| ");
        header_line.push_str("-".repeat(*max_length).as_str());
        header_line.push(' ');
    }
    header_line.push('|');
    header_line
}

fn calc_max_column_length(header: &[String], data: &[Vec<String>]) -> Vec<usize> {
    let mut max_len: Vec<usize> = vec![0; data[0].len()];

    for (i, col) in header.iter().enumerate() {
        if max_len[i] < col.len() {
            max_len[i] = col.len();
        }
    }

    for row in data {
        for (i, col) in row.iter().enumerate() {
            if max_len[i] < col.len() {
                max_len[i] = col.len();
            }
        }
    }

    max_len
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_to_string_rows_empty() {
        let epochs: Vec<date::EpochInfo> = Vec::new();
        let timezones: Vec<arg::Zone> = Vec::new();

        let (h, d) = to_string_rows_from_epochs(&epochs, &timezones);
        assert_eq!(1, h.len());
        assert_eq!(0, d.len());
    }
}

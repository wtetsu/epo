use chrono::{FixedOffset, TimeZone, Utc};

fn main() {
    if std::env::args().len() <= 1 {
        eprintln!("Usage: epo utc_offset epoch1 epoch2 ...");
        return;
    }

    let args: Vec<_> = std::env::args().collect();

    let offset: i32 = args[1].parse().unwrap();

    for i in 2..args.len() {
        let epoch: i64 = args[i].parse().unwrap();
        print_date(epoch, offset);
    }
}

fn print_date(epoch_sec: i64, offset: i32) {
    let dt = Utc
        .timestamp(epoch_sec, 0)
        .with_timezone(&FixedOffset::east(offset * 3600));

    println!("{}", dt.format("%Y-%m-%dT%H:%M:%S%z").to_string());
}

mod date;
mod util;

fn main() {
    let args = parse_arguments(std::env::args().collect());

    if args.epoch_secs.len() >= 1 {
        print_epochs(args.epoch_secs, args.offset_secs)
    }

    if args.dates.len() >= 1 {
        print_dates(args.dates);
    }
}

struct Arguments {
    offset_secs: Vec<i32>,
    epoch_secs: Vec<i64>,
    dates: Vec<date::DateValue>,
}

fn parse_arguments(args: Vec<String>) -> Arguments {
    if args.len() <= 1 {
        return make_default_arguments();
    }

    let mut offset_secs: Vec<i32> = Vec::new();
    let mut epoch_secs: Vec<i64> = Vec::new();
    let mut dates: Vec<date::DateValue> = Vec::new();

    for i in 1..args.len() {
        let arg = &args[i];
        if arg.starts_with("+") || arg.starts_with("-") {
            let offset_hour: i32 = arg[1..].parse().unwrap();
            offset_secs.push(offset_hour * 3600);
        } else if util::is_numeric(arg) {
            let epoch_sec: i64 = arg.parse().unwrap();
            epoch_secs.push(epoch_sec);
        } else {
            let r = date::parse_date_str(arg);
            if r.is_ok() {
                dates.push(r.unwrap());
            } else {
                eprintln!("Invalid date: {}", arg);
            }
        }
    }

    if offset_secs.is_empty() {
        offset_secs.push(0);
    }

    return Arguments {
        offset_secs,
        epoch_secs,
        dates,
    };
}

fn make_default_arguments() -> Arguments {
    let now = date::now();

    return Arguments {
        offset_secs: vec![now.offset_sec, 0],
        epoch_secs: vec![now.epoch_sec],
        dates: vec![],
    };
}

fn print_epochs(epoch_secs: Vec<i64>, offset_secs: Vec<i32>) {
    for epoch in epoch_secs {
        let date_strings = offset_secs
            .iter()
            .map(|o| date::to_date_str(epoch, *o))
            .collect::<Vec<_>>();
        print_date_strings(epoch, date_strings);
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

fn print_dates(dates: Vec<date::DateValue>) {
    for date in dates {
        println!("{:24} {:10}", date.date_str, date.epoch_sec);
    }
}

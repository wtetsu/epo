mod date;
mod util;

fn main() {
    let args = parse_arguments(std::env::args().collect());

    if args.epochs.len() >= 1 {
        print_epochs(args.epochs, args.offsets)
    }

    if args.dates.len() >= 1 {
        print_dates(args.dates);
    }
}

struct Arguments {
    offsets: Vec<i32>,
    epochs: Vec<i64>,
    dates: Vec<date::DateValue>,
}

fn parse_arguments(args: Vec<String>) -> Arguments {
    if args.len() <= 1 {
        return make_default_arguments();
    }

    let mut offsets: Vec<i32> = Vec::new();
    let mut epochs: Vec<i64> = Vec::new();
    let mut dates: Vec<date::DateValue> = Vec::new();

    for i in 1..args.len() {
        let arg = &args[i];
        if arg.starts_with("+") || arg.starts_with("-") {
            offsets.push(arg[1..].parse().unwrap());
        } else if util::is_numeric(arg) {
            let epoch: i64 = arg.parse().unwrap();
            epochs.push(epoch);
        } else {
            let r = date::parse_date_str(arg);
            if r.is_ok() {
                dates.push(r.unwrap());
            } else {
                eprintln!("Invalid date: {}", arg);
            }
        }
    }

    if offsets.is_empty() {
        offsets.push(0);
    }

    return Arguments { offsets, epochs, dates };
}

fn make_default_arguments() -> Arguments {
    let now = date::now();

    return Arguments {
        offsets: vec![now.offset_sec / 3600, 0],
        epochs: vec![now.epoch_sec],
        dates: vec![],
    };
}

fn print_epochs(epochs: Vec<i64>, offsets: Vec<i32>) {
    for epoch in epochs {
        let date_strings = offsets.iter().map(|o| date::to_date_str(epoch, *o)).collect::<Vec<_>>();
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

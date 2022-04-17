mod date;

fn main() {
    if std::env::args().len() <= 1 {
        eprintln!("Usage: epo utc_offset epoch1 epoch2 ...");
        return;
    }

    let args: Vec<_> = std::env::args().collect();

    let mut offset_list: Vec<i32> = Vec::new();
    let mut epoch_list: Vec<i64> = Vec::new();

    for i in 1..args.len() {
        let arg = &args[i];
        if arg.starts_with("+") || arg.starts_with("-") {
            offset_list.push(arg[1..].parse().unwrap());
        } else {
            let epoch: i64 = args[i].parse().unwrap();
            epoch_list.push(epoch);
        }
    }

    if offset_list.is_empty() {
        offset_list.push(0);
    }

    for epoch in epoch_list {
        let date_strings = offset_list
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

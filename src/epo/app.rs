pub struct Settings {
  pub offset_secs: Vec<i32>,
  pub epoch_secs: Vec<i64>,
  pub dates: Vec<super::date::DateInfo>,
}

pub fn parse_arguments(args: Vec<String>) -> Settings {
  if args.len() <= 1 {
    return make_default_arguments();
  }

  let mut offset_secs: Vec<i32> = Vec::new();
  let mut epoch_secs: Vec<i64> = Vec::new();
  let mut dates: Vec<super::date::DateInfo> = Vec::new();

  for i in 1..args.len() {
    let arg = &args[i];
    if arg.starts_with("+") || arg.starts_with("-") {
      let offset_hour: i32 = arg[1..].parse().unwrap();
      offset_secs.push(offset_hour * 3600);
    } else if super::util::is_numeric(arg) {
      let epoch_sec: i64 = arg.parse().unwrap();
      epoch_secs.push(epoch_sec);
    } else {
      let r = super::date::parse_date_str(arg);
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

  return Settings {
    offset_secs,
    epoch_secs,
    dates,
  };
}

pub fn make_default_arguments() -> Settings {
  let now = super::date::now();

  return Settings {
    offset_secs: vec![now.offset_sec, 0],
    epoch_secs: vec![now.epoch_sec],
    dates: vec![],
  };
}

pub fn print_all(settings: Settings) {
  if settings.epoch_secs.len() >= 1 {
    print_epochs(settings.epoch_secs, settings.offset_secs)
  }

  if settings.dates.len() >= 1 {
    print_dates(settings.dates);
  }
}

fn print_epochs(epoch_secs: Vec<i64>, offset_secs: Vec<i32>) {
  for epoch in epoch_secs {
    let date_strings = offset_secs
      .iter()
      .map(|o| super::date::to_date_str(epoch, *o))
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

fn print_dates(dates: Vec<super::date::DateInfo>) {
  for date in dates {
    println!("{:24} {:10}", date.date_str, date.epoch_sec);
  }
}

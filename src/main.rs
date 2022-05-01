mod epo;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    match epo::arg::parse_arguments(&args) {
        Ok(settings) => epo::app::run(&settings),
        Err(errors) => print_errors(&errors),
    }
}

fn print_errors(errors: &Vec<String>) {
    for error in errors {
        eprintln!("{}", error);
    }
}

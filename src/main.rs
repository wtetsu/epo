mod epo;
use std::process::exit;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    match epo::arg::parse_arguments(&args) {
        Ok(settings) => epo::app::run(&settings),
        Err(errors) => {
            print_errors(&errors);
            exit(1);
        }
    }
}

fn print_errors(errors: &Vec<String>) {
    for error in errors {
        eprintln!("{}", error);
    }
}

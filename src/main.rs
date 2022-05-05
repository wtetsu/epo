mod epo;
use std::process::exit;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let parse_settings = epo::arg::get_parse_settings();
    match epo::arg::parse_arguments(&args, &parse_settings) {
        Ok(app_settings) => epo::app::run(&app_settings),
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

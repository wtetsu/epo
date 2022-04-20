mod epo;

fn main() {
    let r = epo::app::parse_arguments(std::env::args().collect());
    match r {
        Ok(settings) => epo::app::run(settings),
        Err(errors) => print_errors(errors),
    }
}

fn print_errors(errors: Vec<String>) {
    for error in errors {
        eprintln!("{}", error);
    }
}

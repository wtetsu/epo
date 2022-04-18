mod epo;

fn main() {
    let args = epo::app::parse_arguments(std::env::args().collect());
    epo::app::run(args);
}

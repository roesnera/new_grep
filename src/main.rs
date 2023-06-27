use std::env;
pub mod lib;
use lib::run;
fn main() {
    let args: Vec<String> = env::args().collect();
    let config  = Config::new(&args);

    let query = config.query;
    let location = config.location;
    run(query, location);
}

struct Config {
    query: String,
    location: String
}

impl Config {
    fn new(args: &[String]) -> Config {
        if args.len() < 3 {
            panic!("two arguments required");
        }
        let query = args[1].clone();
        let location = args[2].clone();

        Config {query, location}
    }
}


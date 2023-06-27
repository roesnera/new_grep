use std::env;
//pub mod lib;
//use lib::run;
use std::process;
use std::fs;
fn main() {
    let args: Vec<String> = env::args().collect();
    //let config  = Config::new(&args);

    let config = Config::build(&args).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {err}");
        process::exit(1);
    });

    run(config);
}

pub struct Config {
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
    fn build(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("not enough arguments");
        }
        let query = args[1].clone();
        let location = args[2].clone();

        Ok(Config { query, location })
    }
}

pub fn run(config: Config) {

    println!("searching for {} in {}",config.query,config.location);
    let contents = fs::read_to_string(config.location)
        .expect("should have been able to read the file!");

    println!("with text:\n{contents}");
}

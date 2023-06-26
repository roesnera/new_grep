use std::env;
use std::fs;
fn main() {
    let args: Vec<String> = env::args().collect();
    dbg!(&args);
    let query = &args[1];
    let location = &args[2];

    println!("Searching for {query} in {location}");

    let contents = fs::read_to_string(location)
        .expect("Should have been able to read the file!");

    println!("With text:\n{contents}");
}

use std::env;
fn main() {
    let args: Vec<String> = env::args().collect();
    dbg!(&args);
    let query = &args[1];
    let location = &args[2];

    println!("Searching for {query} in {location}");
}

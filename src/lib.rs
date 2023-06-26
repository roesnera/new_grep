use std::fs;
pub fn run(query: String, location: String) {



    println!("Searching for {query} in {location}");
    let contents = fs::read_to_string(location)
        .expect("Should have been able to read the file!");

    println!("With text:\n{contents}");
}

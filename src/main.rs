use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();
    let query = &args[1];
    let filename = &args[2];
    println!("Searching for {}", query);
    println!("In file {}", filename);
    // dbg!(args);
    let contents = fs::read_to_string(filename).expect("Should have been able to read the file");

    println!("With text:\n{contents}");
}

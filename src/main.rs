// it's convention to bring in the parent module in scope, rather than the fn
// this makes it clearer where fns are coming from
use std::env;
use std::fs;
fn main() {
    // The type on arg tells Rust what kind of collect() to run
    let args: Vec<String> = env::args().collect();

    // println!("{:?}", args);
    //> ["target/debug/grup", "needle", "haystack"]

    let query = &args[1];
    let filename = &args[2];

    println!("Searching for {}", query);
    println!("In file {}", filename);

    let contents = fs::read_to_string(filename)
    .expect(&format!("Something went wrong reading the file {}", filename));

    println!("With text:\n{}", contents);
}
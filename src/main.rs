// it's convention to bring in the parent module in scope, rather than the fn
// this makes it clearer where fns are coming from
use grup::Config;
use std::env;
use std::process;
fn main() {
    // The type on arg tells Rust what kind of collect() to run

    let config = Config::new(env::args()).unwrap_or_else(|err| {
        eprintln!("Problem parsing args: {}", err);
        process::exit(1);
    });

    if let Err(e) = grup::run(config) {
        eprintln!("Application error {}", e);
        process::exit(1);
    }
}

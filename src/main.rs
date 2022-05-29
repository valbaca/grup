// it's convention to bring in the parent module in scope, rather than the fn
// this makes it clearer where fns are coming from
use std::env;
use std::error::Error;
use std::fs;
use std::process;
fn main() {
    // The type on arg tells Rust what kind of collect() to run
    let args: Vec<String> = env::args().collect();

    let config = Config::new(&args).unwrap_or_else(|err| {
        println!("Problem parsing args: {}", err);
        process::exit(1);
    });

    if let Err(e) = run(config) {
        println!("Application error {}", e);
        process::exit(1);
    }
}

fn run(config: Config) -> Result<(), Box<dyn Error>> {
    println!("Searching for {}", config.query);
    println!("In file {}", config.filename);

    let contents = fs::read_to_string(config.filename)?;

    println!("With text:\n{}", contents);
    Ok(())
}

struct Config {
    query: String,
    filename: String,
}

impl Config {
    fn new(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("not enough args");
        }
        let query = args[1].clone();
        let filename = args[2].clone();
        Ok(Config { query, filename })
    }
}

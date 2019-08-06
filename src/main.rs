
use std::env;

use minigrep::Config;

fn main() {
    // Collect command line arguments.
    let args: Vec<String> = env::args().collect();

    // Parse command line arguments.
    let config = Config::new(&args).unwrap_or_else(|err| {
        eprintln!("{}", err);
        eprintln!("Usage: {} `needle` `haystack.txt`", args[0]);
        std::process::exit(1);
    });

    // Start the program.
    if let Err(e) = minigrep::run(config) {
        eprintln!("Application Error: {}", e);
        std::process::exit(1);
    }
}

use std::{env, process};

use minigrep::Config;

fn main() {
    let args: Vec<String> = env::args().collect();

    let config = Config::new(&args).unwrap_or_else(|error| {
        eprintln!("Error preparing config: {}", error);
        process::exit(1)
    });

    println!("{:#?}", config);

    minigrep::run(config).unwrap_or_else(|error| {
        eprintln!("Error running: {}", error);
        process::exit(1)
    });
}

use std::env;

use minigrep::Config;

fn main() {
    let args: Vec<String> = env::args().collect();

    let config = Config::new(args);

    println!("{:#?}", config);

    minigrep::run(config);
}

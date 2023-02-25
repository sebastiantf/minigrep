use std::env;

use minigrep::Config;

fn main() {
    let args: Vec<String> = env::args().collect();

    let config = Config::new(args[1].clone(), args[2].clone());

    println!("{:#?}", config);

    minigrep::run(config);
}

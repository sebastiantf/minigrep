use std::env;

use minigrep::Config;

fn main() {
    let args: Vec<String> = env::args().collect();

    let case_sensitive = env::var("CASE_SENSITIVE").is_ok();

    let config = Config::new(args[1].clone(), args[2].clone(), case_sensitive);

    println!("{:#?}", config);

    minigrep::run(config);
}

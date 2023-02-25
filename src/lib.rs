use std::fs;

#[derive(Debug)]
pub struct Config {
    query: String,
    filename: String,
}

impl Config {
    pub fn new(query: String, filename: String) -> Config {
        Config { query, filename }
    }
}

pub fn run(config: Config) {
    let lines = fs::read_to_string(config.filename);
    for line in lines {
        println!("{}", line);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
}

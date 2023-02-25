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

#[cfg(test)]
mod tests {
    use super::*;
}

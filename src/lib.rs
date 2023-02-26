use std::env;
use std::error::Error;
use std::fs;

#[derive(Debug)]
pub struct Config {
    query: String,
    filename: String,
    case_sensitive: bool,
}

impl Config {
    // we have an owned value here, so compiler cannot set any lifetime for the returned &str automatically, since there are no reference inputs
    // iterator should be mutable
    pub fn new(mut args: env::Args) -> Result<Config, &'static str> {
        // prevent index out of bounds error
        if args.len() < 3 {
            return Err("Not enough args");
        }

        // skip first arg
        args.next();

        let query = if let Some(query) = args.next() {
            query
        } else {
            return Err("No arg for query");
        };
        let filename = if let Some(filename) = args.next() {
            filename
        } else {
            return Err("No arg for filename");
        };

        let case_sensitive = env::var("CASE_SENSITIVE").is_ok();

        Ok(Config {
            query,
            filename,
            case_sensitive,
        })
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    // propagate error instead of panic
    let lines = fs::read_to_string(config.filename)?;

    let results = if config.case_sensitive {
        search(&lines, &config.query)
    } else {
        search_case_insensitive(&lines, &config.query)
    };

    for lines in results {
        println!("{}", lines);
    }

    Ok(())
}

fn search<'a>(contents: &'a str, query: &str) -> Vec<&'a str> {
    let mut result: Vec<&str> = Vec::new();

    for line in contents.lines() {
        if line.contains(query) {
            result.push(line);
        }
    }

    result
}

fn search_case_insensitive<'a>(contents: &'a str, query: &str) -> Vec<&'a str> {
    let mut result: Vec<&str> = Vec::new();

    for line in contents.lines() {
        if line.to_lowercase().contains(&query.to_lowercase()) {
            result.push(line);
        }
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_search() {
        let query = "body";
        let contents = "\
I'm nobody! Who are you?
Are you nobody, too?
Then there's a pair of us - don't tell!
They  'd banish us, you know.
How dreary to be somebody!
How public, like a frog
To tell your name the livelong day
To an admiring bog!";

        assert_eq!(
            vec![
                "I'm nobody! Who are you?",
                "Are you nobody, too?",
                "How dreary to be somebody!"
            ],
            search(contents, query)
        );
    }

    #[test]
    fn test_search_case_insensitive() {
        let query = "body";
        let contents = "\
I'm nobody! Who are you?
Are you nobody, too?
Then there's a pair of us - don't tell!
They  'd banish us, you know.
How dreary to be somebody!
How public, like a frog
Body
body
Every Body
To tell your name the livelong day
To an admiring bog!";

        assert_eq!(
            vec![
                "I'm nobody! Who are you?",
                "Are you nobody, too?",
                "How dreary to be somebody!",
                "Body",
                "body",
                "Every Body"
            ],
            search_case_insensitive(contents, query)
        );
    }
}

use std::fs;
use std::env;
use std::error::Error;

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.filename)?;

    let results = if config.case_sensitive {
        search(&config.query, &contents)
    } else {
        search_case_insensitive(&config.query, &contents)
    };

    for line in results {
        println!("{}", line);
    }
    Ok(())
}

fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let mut result = vec![];
    for line in contents.lines() {
        let line = line.trim();
        if line.contains(query) {
            result.push(line);
        }
    }
    result
}

fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let query = &query.to_lowercase();
    let mut result = vec![];
    for line in contents.lines() {
        let line = line.trim();
        if line.to_lowercase().contains(query) {
            result.push(line);
        }
    }
    result
}


pub struct Config {
    pub query: String,
    pub filename: String,
    pub case_sensitive: bool,
}

impl Config {
    pub fn new(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("not enough arguments");
        }
        let query = args[1].clone();
        let filename = args[2].clone();
        let case_sensitive = env::var("CASE_INSENSITIVE").is_err();
        Ok(Config { query, filename, case_sensitive})
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn one_result() {
        let query = "duct";
        let contents = "\n
            Rust:\n
            safe, fast, productive.\n
            Pick three.";

        assert_eq!(
            vec!["safe, fast, productive."],
            search(query, contents)
        )
    }

    #[test]
    fn case_sensitive() {
        let query = "duct";
        let contents = "\n
            Rust:\n
            safe, fast, productive.\n
            Pick three.\n\
            Duct tape.";

        assert_eq!(
            vec!["safe, fast, productive."],
            search(query, contents)
        )
    }

    #[test]
    fn case_insensitive() {
        let query = "rUsT";
        let contents = "\n
            Rust:\n
            safe, fast, productive.\n
            Pick three.\n\
            Trust me.";

        assert_eq!(
            vec!["Rust:", "Trust me."],
            search_case_insensitive(query, contents)
        )
    }
}
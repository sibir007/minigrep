use std::fs;
use std::error::Error;
use std::env;

pub fn run(config: Config) -> Result<(), Box<dyn Error>>{

    let contents = fs::read_to_string(config.file_path)?;

    let results = if config.case_sensitive {
        search(&config.query, &contents)
    } else {
        search_insensitive(&config.query, &contents)
    };

    for line in results {
        println!("{}", line);
    }

    // println!("With text:\n{contents}");

    Ok(())
}

pub struct Config {
    pub query: String,
    pub file_path: String,
    pub case_sensitive: bool,
}

impl Config {

    pub fn build(mut args: impl Iterator<Item = String>) -> Result<Config, &'static str> {

        args.next();

        let query: String = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn't get a query string"),
        };

        let file_path: String = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn't get a file path"),
        };


        let case_sensitive = env::var("CASE_SENSITIVE").is_ok();
        
        Ok(Config { query, file_path, case_sensitive })

    }
}

pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    
    contents.lines().filter(|line| line.contains(query)).collect()
}

pub fn search_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    
    let query: &str = &query.to_lowercase();
    contents.lines()
    .filter(|line| line.to_lowercase().contains(query))
    .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_sensitive() {
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.";

        assert_eq!(vec!["safe, fast, productive."], search(query, contents));
    }

    #[test]
    fn case_insensitive() {
        let query = "rUsT";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.";

        assert_eq!(vec!["Rust:"], search_insensitive(query, contents));
    }
}
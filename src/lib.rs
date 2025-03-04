use std::fs;
use std::error::Error;
use std::env;

pub fn run(config: Config) -> Result<(), Box<dyn Error>>{
    // println!("Searching for {}", config.query);
    // println!("In file {}", config.file_path);
    // dbg!(args);
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

    pub fn build(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return  Err("not enough arguments");
        }

        let query = args[1].clone();
        let file_path = args[2].clone();
        let case_sensitive = env::var("CASE_SENSITIVE").is_ok();
        
        Ok(Config { query, file_path, case_sensitive })

    }
}

pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    
    let mut results: Vec<&str>  = Vec::new();
    
    for line in contents.lines() {
        if line.contains(query) {
            results.push(line);
            // println!("{}", line);
        }
        
    }
    results
}

pub fn search_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    
    let query = query.to_lowercase();
    let mut results: Vec<&str>  = Vec::new();
    
    for line in contents.lines() {
        if line.to_lowercase().contains(&query) {
            results.push(line);
            // println!("{}", line);
        }
        
    }
    results
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
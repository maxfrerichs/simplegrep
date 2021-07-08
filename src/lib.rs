use std::fs;
use std::error::Error;
use std::env;

pub struct Config {
    query: String,
    filename: String,
    case_sensitive: bool
}

impl Config {
    pub fn new(args: &[String]) -> Result<Config, &str> {
        if args.len() > 3 {
            return Err("Too many arguments");
        }
        if args.len() < 3 {
            return Err("Too few arguments");
        }
        let case_sensitive = env::var("CASE_INSENSITIVE").is_err();
        return Ok(Config {
            query: args[1].clone(),
            filename: args[2].clone(),
            case_sensitive: case_sensitive
        });
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    println!("In file {}: ", config.filename);
    let file_contents = fs::read_to_string(config.filename).expect("Something went wrong...");
    if config.case_sensitive {
        for search_result in search(&config.query, &file_contents) {
            println!("{}", search_result)
        }
    } else {
        for search_result in search_case_insensitive(&config.query, &file_contents) {
            println!("{}", search_result)
        }
    }

    Ok(())
}

pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let mut results = vec![];
    for line in contents.lines() {
        if line.contains(query) {
            results.push(line);
        }
    }
    return results;
}

pub fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let mut results = vec![];
    let query = query.to_lowercase();
    for line in contents.lines() {
        if line.to_lowercase().contains(&query) {
            results.push(line);
        }
    }
    return results;
}
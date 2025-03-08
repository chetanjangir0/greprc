use std::fs;
use std::error::Error;
use std::env;

pub struct Config {
    pub query:String,
    pub filename:String,
    pub case_sensitive:bool,
}

impl Config {
    pub fn new(args: &[String]) -> Result<Self, &str> {
        if args.len() < 3 {
            return Err("not enough arguments");
        }
        let query = args[1].clone();
        let filename = args[2].clone();

        let case_sensitive = env::var("CASE_INSENSITIVE").is_err();
    
        Ok(Self {
            query, 
            filename, 
            case_sensitive
        })
        
    }   
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>>{
    let file_content = fs::read_to_string(config.filename)?;

    let search_res = if config.case_sensitive {
        search(&config.query, &file_content)
    } else {
        search_case_insensitive(&config.query, &file_content)
    };

    for line in search_res {
        println!("{}", line);
    }
    Ok(())
}

fn search<'a>(query:&str, content:&'a str) -> Vec<&'a str> {

    let mut res = Vec::new();
    for line in content.lines() {
        if line.contains(query) {
            res.push(line);
        }
    }

    res
}

fn search_case_insensitive<'a>(query:&str, content:&'a str) -> Vec<&'a str> {
    let query = query.to_lowercase(); // shadowing also converts to String

    let mut res = Vec::new();
    for line in content.lines() {
        if line.to_lowercase().contains(&query) {
            res.push(line);
        }
    }

    res
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn case_sensitive() {
        let query = "duct";
        let content = "\nRust:\nsafe, fast, productive.\nDuct tape.";

        assert_eq!(
            vec!["safe, fast, productive."], 
            search(query, content)
        );
    }

    #[test]
    fn case_insensitive() {
        let query = "rUsT";
        let content = "Rust:\nsafe, fast, productive.\nPick three.\nTrust me.";
        assert_eq!(
            vec!["Rust:", "Trust me."],
            search_case_insensitive(query, content)
        );
    }
}
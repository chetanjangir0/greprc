use std::fs;
use std::error::Error;

pub struct Config {
    pub query:String,
    pub filename:String,
}

impl Config {
    pub fn new(args: &[String]) -> Result<Self, &str> {
        if args.len() < 3 {
            return Err("not enough arguments");
        }
        let query = args[1].clone();
        let filename = args[2].clone();
    
        Ok(Self {query, filename})
        
    }   
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>>{
    let file_content = fs::read_to_string(config.filename)?;
    println!("the file content is: {}", file_content);
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

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn one_result() {
        let query = "duct";
        let content = "\nRust:\nsafe, fast, productive.\nPick three.";

        assert_eq!(
            vec!["safe, fast, productive."], 
            search(query, content)
        );
    }
}
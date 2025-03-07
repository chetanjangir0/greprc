use std::env;
use std::fs;
use std::process;
fn main() {
    let args:Vec<String> = env::args().collect();

    let config = Config::new(&args).unwrap_or_else(|e| {
        println!("problem parsing arguments: {}", e);
        process::exit(1);
    });
    
    let file_content = fs::read_to_string(config.filename).expect("file not found");

    println!("the file content is: {}", file_content);
}

struct Config {
    query:String,
    filename:String,
}

impl Config {
    fn new(args: &[String]) -> Result<Self, &str> {
        if args.len() < 3 {
            return Err("not enough arguments");
        }
        let query = args[1].clone();
        let filename = args[2].clone();
    
        Ok(Self {query, filename})
        
    }   
}
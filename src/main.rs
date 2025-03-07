use std::env;
use std::fs;
use std::process;
use std::error::Error;
fn main() {
    let args:Vec<String> = env::args().collect();

    let config = Config::new(&args).unwrap_or_else(|e| {
        println!("problem parsing arguments: {}", e);
        process::exit(1);
    });
    
    if let Err(e) = run(config){ // not using unwrap_or_else cuz run returns a () type
        println!("Application error: {}", e);
        process::exit(1);
    };
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

fn run(config: Config) -> Result<(), Box<dyn Error>>{
    let file_content = fs::read_to_string(config.filename)?;
    println!("the file content is: {}", file_content);
    Ok(())
}
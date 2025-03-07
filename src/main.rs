use std::env;
use std::fs;
fn main() {
    let args:Vec<String> = env::args().collect();

    let config = Config::new(&args);
    
    let file_content = fs::read_to_string(config.filename).expect("file not found");

    println!("the file content is: {}", file_content);
}

struct Config {
    query:String,
    filename:String,
}

impl Config {
    fn new(args: &[String]) -> Self {
        let query = args[1].clone();
        let filename = args[2].clone();
    
        Self {query, filename}
    }   
}
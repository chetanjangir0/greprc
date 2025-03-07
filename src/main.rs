use std::env;
use std::fs;
fn main() {
    let args:Vec<String> = env::args().collect();

    let query = &args[1];
    let filename = &args[2];
    
    let file_content = fs::read_to_string(filename).expect("the file doesn't exist in the current directory");

    println!("the file content is: {}", file_content);
}

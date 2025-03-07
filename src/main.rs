extern crate greprs;
use greprs::Config;

use std::env;
use std::process;
fn main() {
    let args:Vec<String> = env::args().collect();

    let config = Config::new(&args).unwrap_or_else(|e| {
        println!("problem parsing arguments: {}", e);
        process::exit(1);
    });
    
    if let Err(e) = greprs::run(config){ // not using unwrap_or_else cuz run returns a () type
        println!("Application error: {}", e);
        process::exit(1);
    };
}
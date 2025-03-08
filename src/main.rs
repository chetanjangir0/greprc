extern crate greprs;
use greprs::Config;

use std::env;
use std::process;
use std::io::prelude::*;
fn main() {
    let args:Vec<String> = env::args().collect();
    let mut stderr = std::io::stderr();

    let config = Config::new(&args).unwrap_or_else(|e| {
        writeln!(
            &mut stderr, 
            "problem parsing arguments: {}", e
        ).expect("could not write to stderr");
        process::exit(1);
    });
    
    if let Err(e) = greprs::run(config){ // not using unwrap_or_else cuz run returns a () type
        writeln!(
            &mut stderr, "Application error: {}", e
        ).expect("could not write to stderr");
        process::exit(1);
    };
}
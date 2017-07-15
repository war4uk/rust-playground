extern crate greprs;

use std::env;
use std::process;
use std::error::Error;

fn main() {
    let args: Vec<String> = env::args().collect();

    let config = greprs::Config::new(&args).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {}", err);
        process::exit(1)
    });

    if let Err(e) = greprs::run(config) {
        println!("Application error: {}", e);
        process::exit(1);
    }
}

extern crate simplegrep;

use simplegrep::Config;
use std::process;
use std::env;

fn main() {
    let arguments: Vec<String> = env::args().collect();
    let config = Config::new(&arguments).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {}", err);
        process::exit(1);
    });
    if let Err(e) = simplegrep::run(config) {
        eprintln!("Application error: {}", e);
        process::exit(1);
    }
     
}



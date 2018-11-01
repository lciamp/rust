//#![allow(unused)]
extern crate minigrep;

use std::env;  // to get command line args
use std::process;
use minigrep::Config;

fn main() {

    
    let config = Config::new(env::args()).unwrap_or_else(|err| {
        eprintln!("Problem parsing args: {}", err);
        process::exit(1);
    });

    if let Err(e) = minigrep::run(config) {
        eprintln!("Application error: {}", e);
        process::exit(1);
    }



}


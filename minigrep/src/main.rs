//#![allow(unused)]
extern crate minigrep;

use std::env;  // to get command line args
use std::process;
use minigrep::Config;

fn main() {

    let args: Vec<String> = env::args().collect();

    let config = Config::new(&args[1..]).unwrap_or_else(|err| {
        eprintln!("Problem parsing args: {}", err);
        process::exit(1);
    });

    if let Err(e) = minigrep::run(config) {
        eprintln!("Application error: {}", e);
        process::exit(1);
    }



}


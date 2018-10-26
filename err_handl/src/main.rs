#![allow(unused)]

use std::fs::File;
use std::io;
use std::io::ErrorKind;
use std::io::Read;

fn main() {
    let f = File::open("hello.txt");

    let f = match f {
        Ok(file) => file,
        Err(ref error) if error.kind() == ErrorKind::NotFound => 
        	match File::create("hello.txt") {
            	Ok(fc) => fc,
            	Err(e) => panic!("Tried to create file but there was a problem: {:?}", e),
        	},
        Err(error) => panic!("There was a problem opening the file: {:?}", error),
    };

    let f2 = File::open("hello.txt").expect("There was an error opening hello.txt.");

    println!("{:?}", read_username_better().unwrap());
}

fn read_username() -> Result<String, io::Error> {
	let f = File::open("hello.txt");

	let mut f = match f {
		Ok(file) => file,
		Err(error) => return Err(error),
	};

	let mut s = String::new();

	match f.read_to_string(&mut s){
		Ok(_) => Ok(s),
		Err(e) => Err(e),
	}
}

fn read_username_better() -> Result<String, io::Error> {
	let mut f = File::open("hello.txt")?;
	let mut s = String::new();
	f.read_to_string(&mut s)?;
	Ok(s)
}

fn read_username_even_better() -> Result<String, io::Error> {
	let mut s = String::new();
	File::open("hello.txt")?.read_to_string(&mut s)?;
	Ok(s)
}
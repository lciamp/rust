#![allow(unused)]

use std::thread;
use std::time::Duration;

fn main() {

    let handel = thread::spawn(|| {
        for i in 0..10 {
            println!("hi number {} from spawned thread!", i);
            thread::sleep(Duration::from_millis(1));
        }
    });

    for i in 1..5 {
        println!("hi number {} form main thread!", i);
        thread::sleep(Duration::from_millis(1));
    }

    handel.join().unwrap();
}
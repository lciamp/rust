#![allow(unused)]

use std::sync::mpsc;
use std::thread;
use std::time::Duration;


fn main() {

    let (tx, rx) = mpsc::channel();
    let tx1 = mpsc::Sender::clone(&tx);

    thread::spawn(move || {
        let values = vec![
            String::from("hello"),
            String::from("from"),
            String::from("the"),
            String::from("thread"),
        ];

        for val in values {
            tx1.send(val).unwrap();
            thread::sleep(Duration::from_millis(1000));
        }
    });

    thread::spawn(move || {
        let values = vec![
            String::from("more"),
            String::from("messages"),
            String::from("for"),
            String::from("you"),
        ];

        for val in values {
            tx.send(val).unwrap();
            thread::sleep(Duration::from_millis(1000));
        }
    });


    for recieved in rx {
        println!("Got: {}", recieved);
    }
}
#![allow(unused)]
fn main() {

    let mut new_s = String::new();

    let s = "initial content".to_string();

    let mut s = String::from("foo");

    s.push_str("bar");

    let s1 = String::from("hello, ");
    let s2 = String::from("world.");
    let s3 = s1 + &s2;

    println!("{}", s3);

    let s1 = "tic".to_string();
    let s2 = "tac".to_string();
    let s3 = "toe".to_string();

    let game = format!("{} - {} - {}", s1, s2, s3);
    println!("{}", game);

    let chars = game.chars();
    let mut bytes = game.bytes();

    let a = game.chars();
    let mut b = game.bytes();

}
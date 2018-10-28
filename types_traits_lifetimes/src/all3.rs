#![allow(unused)]

use std::fmt::Display;

fn main() {
    let string1 = "abcd";
    let string2 = "xyz";
    let string3 = "hello";
    println!("the longest: {}",longest_w_announcement(string1, string2, string3));

}

fn longest_w_announcement<'a, T>(x: &'a str, y: &'a str, ann: T) -> &'a str
    where T: Display
{
    println!("Announcement! {}", ann);
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

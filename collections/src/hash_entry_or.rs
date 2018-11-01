#![allow(unused)]

use std::collections::HashMap;

fn main() {
    let mut map = HashMap::new();

    *map.entry("lou").or_insert(2) += 5;
    println!("{:?}", &map);

    *map.entry("lou").or_insert(99) += 5;
    *map.entry("hudson").or_insert(2);

    println!("{:?}", &map);

    let text = "hello world wonderful world";
    let mut map = HashMap::new();

    for word in text.split_whitespace() {
        *map.entry(word).or_insert(0) += 1;
    }

    println!("{:?}", map);

}



#![allow(unused)]

use std::collections::HashMap;

fn main() {
    let mut map = HashMap::new();

    *map.entry("lou").or_insert(2) += 5;
    println!("{:?}", map);

    *map.entry("lou").or_insert(99) += 5;
    *map.entry("hudson").or_insert(2);

    println!("{:?}", map);



}
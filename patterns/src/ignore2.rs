#![allow(unused)]

struct Point {
    x: i32,
    y: i32, 
    z: i32,
}

fn main() {

let origin = Point {x: 0, y: 0, z: 0};

match origin {
    Point{x, ..} => println!("x is {}", x),
}

let numbers = (1, 2, 3, 4, 5);

match numbers {
    (first, .., last) => println!("first: {}, last: {}", first, last),
}

}
#![allow(unused)]

// using ref to mkae references so ownership is not moved
// in to the pattern

fn main() {

    let mut robot_name = Some(String::from("bors"));

    match robot_name {
        Some(ref name) => println!("found a name: {}", name),
        None => (),
    }
    
    println!("robot name: {}", robot_name.clone().unwrap());

    match robot_name {
        Some(ref mut name) => *name = String::from("hudson"),
        None => (),
    }

    println!("robot name: {}", robot_name.unwrap());

}
#![allow(unused)]


use std::io::Write;
use std::str;

fn main() {
    let number_list = vec![34, 50, 25, 100, 65];

    let mut largest = number_list[0];

    for number in number_list {
        if number > largest {
            largest = number;
        }
    }

    println!("The largest number is: {}", largest);

    let number_list = vec![34, 50, 250, 100, 65];

    println!("The largest number is: {:?}", largest_fn(&number_list));

    let char_list = vec!['a', 'b', 'c', 't', 'r', 's'];

    println!("The largest number is: {:?}", largest_fn(&char_list));

    let mut w = Vec::new();
    write!(w, "Hello {}!", "world");

    let test = str::from_utf8(&w).unwrap();

    println!("{}", &test);
}

fn largest_fn<T: PartialOrd + Copy>(list: &[T]) -> T {
    let mut largest = list[0];

    for &item in list.iter() {
        if item > largest {
            largest = item;
        }
    }
    largest
}

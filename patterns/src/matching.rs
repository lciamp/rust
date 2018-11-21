#![allow(unused)]

fn main() {

    let a_value: Option<i32> = Some(3);
    
    if let Some(x) = a_value {
        println!("{}", x);
    }
    /* NOT ALLOWED:
    if let x = 5 {
        println!("{}", x);
    }
    */

    // MATCHING LITERALS:
    let x = 1;
    match x {
        1 => println!("one"),
        2 => println!("two"),
        3 => println!("three"),
        _ => println!("anything"),
    }

    // MATCHING NAMED VARIABLES
    let x = Some(8);
    let y = 10;
    match x {
        Some(50) => println!("got 50"),
        Some(y) => println!("Matched, y = {:?}", y),
        _ => println!("Default case, x = {:?}", x),
    }

    println!("at the end: x = {:?}, y = {:?}", x, y);


    // MULTIPLE PATTERNS
    let x = 1;

    match x {
        1 | 2 => println!("one or two"),
        3 => println!("three"),
        _ => println!("anything"),
    }

    // MATCHING WITH RANGE
    let x = 5;

    match x {
        1...5 => println!("one through five"),
        _ => println!("anything"),
    }

    let x = 'c';
    match x {
        'a'...'j' => println!("earily ASCII letter."),
        'k'...'z' => println!("late ASCII letter."),
        _ => println!("something else"),
    }
}
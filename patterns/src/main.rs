#![allow(unused)]

fn main() {

    // IF LET EXPRESSIONS
    let favroite_color: Option<&str> = None;
    let is_tuesday = false;
    let age: Result<u8, _> = "34".parse();

    if let Some(color) = favroite_color {
        println!("Using: {} as a background color.", color);
    } else if is_tuesday {
        println!("Tuesday is a green day.");
    } else if let Ok(age) = age {
        if age > 30 {
            println!("Using purple as background color.");
        } else {
            println!("using orange as background color.");
        }
    } else {
        println!("using blue as background color");
    }


    let mut stack = Vec::new();

    stack.push(1);
    stack.push(2);
    stack.push(3);

    // WHILE LET CONTITIONAL LOOPS
    while let Some(top) = stack.pop(){
        println!("{}", top);
    }

    let v = vec!['a', 'b', 'c'];

    for (index, value) in v.iter().enumerate() {
        println!("{:?} is at index: {}", value, index);
    }
}
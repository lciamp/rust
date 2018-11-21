#![allow(unused)]

struct Point {
    x: i32,
    y: i32,
}

fn main () {

    let p = Point { x: 0, y: 7 };

    let Point {x: a, y: b} = p;

    assert_eq!(0, a);
    assert_eq!(7, b);

    println!("{:?}", a);

    let Point{ x, y } = p;
    println!("{:?}", x);
    println!("{:?}", y);

    // desturct for matching:

    match p {
        Point{ x, y: 0 } => println!("On the x axis at {}", x),
        Point{ x:0, y } => println!("On the y axis at {}", y),
        Point{ x, y } => println!("On neither axis: ({}, {})", x, y),
    }


    let ((feet, inches), Point{x,y}) = ((3,10), Point{x:3, y:-10});

    println!("feet: {}, inches: {}", feet, inches);
    println!("point x: {}, point y: {}", x, y);
}
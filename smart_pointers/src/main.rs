//#![allow(unused)]



fn main() {

    let b = Box::new(5);

    println!("b: {}", b);


    let x = 5;
    assert_eq!(5, x);

    let y = &x;
    assert_eq!(5, *y);

    let z = Box::new(x);
    assert_eq!(5, *z);

}
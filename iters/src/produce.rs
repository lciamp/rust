#![allow(unused)]


// Iterator adapters: 
fn main() {

    let v1 = vec![1, 2, 3, 4, 5];

    let v2: Vec<_> = v1.iter().map(|x| x+1).collect();

    println!("{:?}", v2);

    let a = [0, 1, 2];

    let mut iter: Vec<&i32> = a.into_iter().filter(|x| **x > 1).collect(); // need two *s!

    println!("{:?}", iter);

}
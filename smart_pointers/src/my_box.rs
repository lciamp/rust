#![allow(unused)]

use std::ops::{Deref, DerefMut};

#[derive(Debug)]
struct MyBox<T>(T);

impl<T> MyBox<T> {
    fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }
}
impl <T> Deref for MyBox<T> {
    type Target = T;

    fn deref(&self) -> &T {
        &self.0
    }
}
impl <T> DerefMut for MyBox<T> {
    fn deref_mut(&mut self) -> &mut T {
        &mut self.0
    }
}


fn main() {

    let x = 5;
    let y = MyBox::new(x);


    assert_eq!(x, 5);
    assert_eq!(*y, 5);

    let m = MyBox::new(String::from("Hudson"));

    hello(&m);


}

fn hello(name: &str) {
    println!("Hello {}", name);
}
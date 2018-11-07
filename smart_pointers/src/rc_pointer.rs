#![allow(unused)]

#[derive(Debug)]
enum List {
    Cons( i32, Rc<List>),
    Nil,
}

use List::{Cons, Nil};
use std::rc::Rc;

fn main() {

    let a = Rc::new(Cons(5,
                Rc::new(Cons(10,
                    Rc::new(Nil)))));
    println!(
        "reference count of a after creating a: {}", 
        Rc::strong_count(&a)
    );

    let b = Cons(3, Rc::clone(&a));
    println!(
        "reference count of a after creating b: {}", 
        Rc::strong_count(&a)
    );

    {
        let c = Cons(4, Rc::clone(&a));
        println!(
            "reference count of a after creating c: {}", 
            Rc::strong_count(&a)
        );
    }

    println!(
        "reference count of a after c goes out of scope: {}", 
        Rc::strong_count(&a)
    );

}
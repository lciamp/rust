#![allow(unused)]

use std::cell::RefCell;
use std::rc::Rc;
use List::{Cons, Nil};

#[derive(Debug)]
enum List {
    Cons(i32, RefCell<Rc<List>>),
    Nil,
}
impl List {
    fn tail(&self) -> Option<&RefCell<Rc<List>>> {
        match *self {
            Cons(_, ref item) => Some(item),
            Nil => None,
        }
    }
}



fn main() {
    
    let a = Rc::new(Cons(5, RefCell::new(Rc::new(Nil))));

    println!("a inital rc count = {}", Rc::strong_count(&a));
    println!("a next item: {:?}", a.tail().unwrap());

    let b = Rc::new(Cons(10, RefCell::new(Rc::clone(&a))));

    println!("a rc count after b creation = {}", Rc::strong_count(&a));
    println!("b inital rc count = {}", Rc::strong_count(&b));
    println!("b next item: {:?}", b.tail().unwrap());
    
    if let Some(link) = a.tail() {
        *link.borrow_mut() = Rc::clone(&b);
    }

    println!("b rc count after chaging a = {}", Rc::strong_count(&b));
    println!("a rc count after changing a = {}", Rc::strong_count(&a));


    // uncomment the next line to see that we have a cycle
    // it will oveflow the stack
    // println!("a next item = {:?}", a.tail());



}
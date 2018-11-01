#![allow(unused)]

use std::collections::HashMap;

struct Cacher<T>
    where T: Fn(u32) -> u32
{
    calculation: T,
    value: HashMap<u32, u32>,
}
impl <T> Cacher<T>
    where T: Fn(u32) -> u32
{
    fn new(calculation: T) -> Cacher<T> {
        Cacher {
            calculation,
            value: HashMap::new(),
        }
    }

    fn value(&mut self, arg: u32) -> u32 {
        *self.value.entry(arg).or_insert((self.calculation)(arg))
    }
}


fn main() {
    let mut test = Cacher::new(|arg| arg+1);

    println!("{}", test.value(2));
    println!("{}", test.value(2));
    println!("{}", test.value(22));
    println!("{}", test.value(22));
}

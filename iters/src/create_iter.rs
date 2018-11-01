#![allow(unused)]

#[derive(Debug)]
struct Counter {
    count: u32,
}

impl Counter {
    fn new() -> Counter {
        Counter { count: 0 }
    }
}

impl Iterator for Counter {
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item> {
        self.count += 1;

        if self.count < 6 {
            Some(self.count)
        } else {
            None
        }
    }
}

fn main() {

    let mut my_counter = Counter::new();

    for i in 0..6 {
        println!("{:#?}", my_counter.next());
    }

    let sum: u32 = Counter::new().zip(Counter::new().skip(1))
                            .map(|(a,b)| a * b)
                            .filter(|x| x % 3 == 0)
                            .sum();

    println!("{}", sum);
        

}
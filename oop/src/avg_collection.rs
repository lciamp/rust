#![allow(unused)]

pub struct AveragedCollection {
    list: Vec<i32>,
    avg: f64,
}

impl AveragedCollection {

    pub fn new() ->  AveragedCollection {
        AveragedCollection {
            list: vec![],
            avg: 0 as f64,
        }
    }

    pub fn add(&mut self, value: i32) {
        self.list.push(value);
        self.update_avg();
    }

    pub fn remove(&mut self) -> Option<i32> {
        let result = self.list.pop();
        match result {
            Some(value) => {
                self.update_avg();
                Some(value)
            },
            None => None,
        }
    }

    pub fn average(&self) -> f64 {
        self.avg
    }

    fn update_avg(&mut self) {
        let total: i32 = self.list.iter().sum();
        self.avg = total as f64 / self.list.len() as f64;
    }
}


fn main() {

    let mut x = AveragedCollection::new();

    x.add(5);
    x.add(3);
    x.add(1);
    x.add(25);
    x.add(37);
    x.add(12);

    println!("avg: {}", x.average());

    println!("removed: {}", x.remove().unwrap());

    println!("avg: {}", x.average());
}
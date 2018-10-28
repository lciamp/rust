#![allow(unused)]

struct Point<T, U>{
    x: T,
    y: U,
}
impl<T, U> Point<T, U>{
    fn x(&self) -> &T {
        &self.x
    }

    fn y(&self) -> &U {
        &self.y
    }
}

impl Point<f32, f32> {
    fn distance_from_origin(&self) -> f32 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }
}

fn main() {

    let integer = Point{x: 2, y: 5};
    let float = Point{x: 2.2, y: 5.5};
    let both = Point{x: 2, y: 5.5};

    println!("x: {}, y: {}", both.x(), both.y());

    println!("distance: {}", float.distance_from_origin());
}
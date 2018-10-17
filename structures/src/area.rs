#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32
}
impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }

    fn square(size: u32) -> Rectangle {
        Rectangle { width: size, height: size }
    }
}

fn main() {
    // example 1:
    let width1 = 30;
    let height1 = 50;

    println!("fn area: {}", area(width1, height1));

    // example with tuple:
    let rect1 = (30, 50);


    println!("fn area_tup: {}", area_tup(rect1));

    // example with structs:
    let my_r = Rectangle { width: 30, height: 50 };
    let my_r2 = Rectangle { width: 20, height: 40 };
    let my_r3 = Rectangle { width: 60, height: 40 };
    println!("my_r can hold my_r2: {}", my_r.can_hold(&my_r2));
    println!("my_r can hold my_r3: {}", my_r.can_hold(&my_r3));

    let my_square = Rectangle::square(20);
    printlxn!("my square: {}", my_square.area());

    println!("fn area_r: {}", area_r(&my_r));

    println!("my_r.area(): {}", my_r.area());

    println!("my_r is: {:#?}", my_r);


}

fn area(width: u32, height: u32) -> u32 {
    width * height
}

fn area_tup(dimensions: (u32, u32)) -> u32 {
    dimensions.0 * dimensions.1
}

fn area_r(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.height
}
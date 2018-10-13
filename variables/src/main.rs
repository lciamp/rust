fn main() {
    let mut x = 5;
    println!("the values of x is: {}", x);
    x = 6;
    println!("the values of x is: {}", x);

    const MAX_POINTS : u32 = 100_000;
    println!("max points: {}", MAX_POINTS);

    let x = 5;

    let x = x + 1;

    let x = x * 2;
    println!("x = {}", x);


    let spaces = "    ";
    let spaces = spaces.len();
    println!("{}", spaces);

    let test = 9.8;
    let test2 = 9_8;
    println!("{} - {}", test, test2);

    let _sum = 5 + 10;
    let _difference = 95.5 - 4.3;
    let _product = 4 * 30;
    let _quotient = 56.7 / 32.2;
    let _remainder = 43 % 5;

    let _t = true;
    let _f: bool = false;

    let tup: (i32, f64, u8)= (500, 6.4, 1);

    let _tup = (500, 6.4, 1);
    let (_x, y, _z) = tup;
    println!("y value: {}", y);


    let x = (500, 6.4, 1);
    let _five_hundred = x.0;
    let _six_point_four = x.1;
    let _one = x.2;

    let a = [1, 2, 3, 4, 5];
    let index = 2;

    println!("index 10: {}", a[index]);

}

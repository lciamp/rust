fn main() {

    let num = -5;
    println!("The {} fib number is: {}", num, fib(num));

}

fn fib(n: i32) -> i32 {
    match n {
        n if n <= 0 => { println!("Invalid Number"); 0},
        n if n <= 1 => n,
        n if n > 1 => fib(n - 1) + fib(n - 2),
        _ => { println!("Invalid"); 0}
    }
}


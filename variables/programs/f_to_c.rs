
// formula = (F − 32) × 5/9
fn main () {

    let temp = 100;
    println!("{}F is {}C", temp, f_to_c(temp));
}

fn f_to_c(f: i32) -> i32 {
    (f-32) * 5/9
}
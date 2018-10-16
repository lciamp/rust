macro_rules! my_macro{
    () => (
        println!("my macro");
    )
}
fn main() {

    my_macro!();

    let s1 = String::from("hello");
    // MOVE, s1 no longer valid, stored on heap
    let mut s2 = s1;

    s2.push_str(", world!");
    println!("{}", s2);

    // CLONE, deep copy, 2 on heap now

    let s1 = String::from("hello");
    let s2 = s1.clone();

    println!("s1 - {}, s2 - {}", s1, s2);

    // stored on stack
    let x = 5;
    let y = x;

    println!("x = {}, y = {}", x, y);

    let s = String::from("hello");

    let mut len = calc_len(&s);

    println!("The length of '{}' is {}", s, len);

    let mut s = String::from("Hello");

    change(&mut s);
    println!("{}", s);

    len = calc_len(&s);

    println!("The length of '{}' is {}", s, len);

}

fn calc_len(s: &String) -> usize {
    s.len()
}

fn change(some_string: &mut String) {
    some_string.push_str(", world.");
}
fn main() {
    let s1 = String::from("hello");

    let mut len = calc_length(&s1); // pass by reference

    println!("The length of '{}' is {}.", s1, len);

    let mut s = String::from("Hello");

    change(&mut s);
    println!("{}", s);

    len = calc_len(&s);

    println!("The length of '{}' is {}", s, len);


    let mut s = String::from("Hello");
    // more than one ref:
    {
        let r1 = &mut s;
    }
    let r2 = &mut s;

    let r1 = &s; // NO PROBLEM
    let r2 = &s; // NO PROBLEM
    let r3 = &mut s; // BIG PROBLEM

}

fn calc_length(s: &String) -> usize {   // ref as arg
    s.len()
}

fn change(some_string: &mut String) {
    some_string.push_str(", world.");
}
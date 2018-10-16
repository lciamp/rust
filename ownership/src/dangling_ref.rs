fn main() {
    let reference_to_nothing = dangle_good();
}

fn dangle() -> &String {    // dangle returns a reference to a string
    let s = String::from("hello");  // s is a new string
    &s  // we return a referent to the String s
}// Here, s goes out of scope, and it dropped. Its memory goes away.

// must return the actual String, ownership is moved out:
fn dangle_good() -> String {
    let s = String::from("hello");
    s
}
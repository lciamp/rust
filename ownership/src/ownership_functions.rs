fn main() {
    let s = String::from("hello"); // s comes into scope

    take_ownership(s); // s's value moves into the function ...
                        // and so is no longer valid here

    println!("{}", s);

    let x = 5; // x comes into scope

    make_copy(x);  // x would move into the function bit i32 is Copy, so it's ok to still
                    // use x afterward
} // here x goes out of scope, then s. But because s's value was moved, nothing special happens

fn take_ownership(some_string: String) {
    println!("{}", some_string);
}// some_string goes out of scope and `drop` is called. The memory is freed

fn make_copy(some_int: i32) {
    println!("{}", some_int);
}// some_int goes out of scope, nothing happens
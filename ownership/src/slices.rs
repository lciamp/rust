fn main() {

    let word = String::from("Hello world");
    let len = first_word(&word);

    println!("first word: {}", len);

    let s = String::from("Hello world");

    // slicing:
    let hello = &s[..5];
    let world = &hello[..2];

    println!("{}, {}, {}", s, hello, world);


}

fn first_word(s: &str) -> &str { // &str is a ref to a string slice
    let bytes = s.as_bytes(); // converts string into array of bytes

    for (i, &item) in bytes.iter().enumerate(){
        if item == b' ' {
            return &s[..i];
        }
    }
    &s[..]
}



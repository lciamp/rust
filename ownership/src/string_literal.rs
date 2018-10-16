fn main() {

    let my_string = String::from("hello world");

    // first_word works on slices of `String s`
    let word = first_word(&my_string[..]);

    println!("{}", word);

    let my_string_literal = "hello world";

    // first_word works on slices of string literals
    let word = first_word(&my_string_literal[..]);

    println!("{}", word);

    // because string literals *are* string slices already,
    // this works too, without the slice syntax
    let word = first_word(my_string_literal);

    println!("{}", word);

    let a = [1,2,3,4,5];
    let slice = &a[1..3];

    println!("{:?}", slice);


}

fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[..i];
        }
    }
    &s[..]
}
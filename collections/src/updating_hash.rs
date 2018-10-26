use std::collections::HashMap;


fn main() {

    let mut scores = HashMap::new();

    // over writing a value
    scores.insert("blue".to_string(), 10);
    scores.insert("blue".to_string(), 25);

    println!("normal inserts: {:?}", scores);

    // only inserting if key has no value
    scores.entry("yellow".to_string()).or_insert(50);
    scores.entry("blue".to_string()).or_insert(50);

    println!("with entry: {:?}", scores);

    // updating value based on 
    let text = "hello world wonderful world";

    let mut map = HashMap::new();

    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0);
        *count += 1
    }

    println!("{:?}", map);
}
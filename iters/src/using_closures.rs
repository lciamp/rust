#![allow(unused)]

#[derive(PartialEq, Debug)]
struct Shoe {
    size: u32,
    style: String,
}

fn main() {

    let shoes = vec![
        Shoe { size: 10, style: "sneaker".to_string() },
        Shoe { size: 13, style: "sandal".to_string() },
        Shoe { size: 10, style: "boot".to_string() },
    ];

    let in_my_size = shoes_in_my_size(shoes, 10);

    println!("{:#?}", in_my_size);

}

fn shoes_in_my_size(shoes: Vec<Shoe>, shoe_size: u32) -> Vec<Shoe> {
    shoes.into_iter()
        .filter(|s| s.size == shoe_size)
        .collect()
}
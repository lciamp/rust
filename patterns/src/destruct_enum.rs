#![allow(unused)]

enum Message {
    Quit,
    Move{x: i32, y: i32},
    Write(String),
    ChangeColor(i32, i32, i32),
}

fn main() {

    let msg = Message::ChangeColor(0, 160, 255);

    match msg {
        Message::Quit => {
            println!("The Quit variant has no data to destructure.")
        },
        Message::Move { x, y }=> {
            println!(
                "Move in the x direction: {} and the y direction: {}", x, y
            );
        },
        Message::Write(text) => {
            println!("Text message: {}", text)
        },
        Message::ChangeColor(r, g, b) => {
            println!(
                "Change color to red {}, green {}, and blue {}", r, g, b
            );
        },
    }

}
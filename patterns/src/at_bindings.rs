#![allow(unused)]


enum Message {
    Hello {id: i32},
}

// @ lets us create a variable that holds a value at the same time we're testing
// that value to see if it matches a pattern
fn main() {
    let msg = Message::Hello{id: 5};

    match msg {
        Message::Hello { id: id_variable @ 3...7 } => {
            println!("Found an id in range: {}", id_variable)
        },
        Message::Hello { id: 10...12 } => {
            println!("Found an id in another range")
        },
        Message::Hello {id} => {
            println!("Found some other id: {}", id)
        },
    }


}
#![allow(unused)]
#[derive(Debug)]
enum UsState {
    Ny,
    Nj,
    Ca,
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

fn main() {

    let my_coin = Coin::Quarter(UsState::Ny);

    let x = value_in_cents(my_coin);

    println!("{}",x);

    let mut count = 0;
    let my_coin = Coin::Quarter(UsState::Nj);
    if let Coin::Quarter(state) = my_coin {
        println!("State quarter from {:?}!", state);
    } else {
        count += 1;
    }

}

fn value_in_cents(coin: Coin) -> u32 {
    match coin {
        Coin::Penny => {
            println!("Lucky Penny!");
            1
        },
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("State quarter from {:?}!", state);
            25
        },
    }
}
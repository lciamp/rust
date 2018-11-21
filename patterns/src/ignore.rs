#![allow(unused)]

fn main() {
    foo(3,4);

    let mut setting_value = None;
    let new_setting_value = Some(10);

    match (setting_value, new_setting_value) {
        (Some(_), Some(_)) => {
            println!("Can't overwrite an exsisting value");
        },
        _ => {
            setting_value = new_setting_value;
        }
    }

    println!("setting is: {:?}", setting_value.unwrap());

    let numbers = (1, 2, 3, 4, 5);

    match numbers {
        (first, _, third, _, fifth) => {
            println!("some numbers: {}, {}, {}", first, third, fifth);
        }
    }

    let s = Some(String::from("hello"));
    // doesn't move s
    if let Some(_) = s {
        println!("found a string");
    }

    println!("{:?}", s.unwrap());

    

}

fn foo(_: i32, y: i32) {
    println!("this code only uses the y param: {}", y);
}
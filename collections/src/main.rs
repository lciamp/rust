#![allow(unused)]

fn main() {

    let mut v = Vec::new();
    v.push(4);
    v.push(5);
    v.push(6);
    v.push(7);

    let new_v = vec![1, 2, 3];

    let third: &i32 = &v[2];
    let third: Option<&i32> = v.get(2);

    println!("printing with Some using .unwrap() : {}", 3 + third.unwrap());

    let test = vec![1, 2, 3, 4, 5];

    //let nope = &test[100];
//    println!("{}", nope);
    let nope = test.get(100);

    let mut v = vec![1,2,3];

    {
        let first = &mut v[0];
        println!("{}", first);
    }
    v.push(6);

    for i in &v {
        println!("{}", i);
    }

    let mut v = vec![1,2,3,4,5];
    for i in &mut v {
        println!("Adding 50 to {}", i);
        // dereference so we can change the value
        *i += 50;
        println!("Now it's: {}", i);
    }



}

fn sort_numbers(arr: &Vec<i32>) -> Vec<i32> {
    let mut answer = arr.clone();
    answer.sort();
    answer
}
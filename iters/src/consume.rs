#![allow(unused)]

fn main() {

    let v1 = vec![1, 2, 3, 4, 5];

    let total: i32 = v1.iter().sum();

    println!("{}", total);

    let v1: Vec<i32> = vec![1, 2, 3, 4, 5];
    let v2: Vec<i32> = vec![1, 2, 3, 4, 5];

    let mut big_iter = v1.iter().chain(v2.iter());
    println!("{:?}", v2);

    let big_vec_refs: Vec<&i32> = big_iter.collect();

    println!("{:?}", big_vec_refs);

    let mut big_vec_no_ref = v1.iter().zip(v2.iter());

    for _ in 0..big_vec_no_ref.len() {
        println!("{:?}", big_vec_no_ref.next());
    }

}
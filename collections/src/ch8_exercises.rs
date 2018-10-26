#![allow(unused)]
use std::collections::HashMap;

fn main() {

    let mut numbers = [1,2,3,4,5,55,8,9,4,23,64,23,44];

    println!("average: {}", average(&numbers));

    println!("mean: {}", mean(&mut numbers));

    println!("mode: {}", mode(&numbers));

}

fn average(numbers: &[i32]) -> f32 {
    numbers.iter().sum::<i32>() as f32 / numbers.len() as f32
}

fn mean (nums: &mut [i32]) -> i32 {
    nums.sort();
    let mid = nums.len() / 2;
    nums[mid]
}
fn mode(numbers: &[i32]) -> i32 {
    let mut occurrences = HashMap::new();

    for &value in numbers {
        *occurrences.entry(value).or_insert(0) += 1;
    }
    println!("{:?}", occurrences);

    occurrences.into_iter()
            .max_by_key(|&(_, count)| count)
            .map(|(val, _)| val)
            .expect("Cannot compute the mode of zero numbers")
}

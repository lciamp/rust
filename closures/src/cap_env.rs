#![allow(unused)]


fn main() {

    /*
    let x = 4;

    let equal_to_x = |z| z == x;

    let y = 4;

    assert!(equal_to_x(x));
    */

    let x = vec![1, 2, 3];

    let equal_to_x = move |z| z == x;


    let y = vec![1,2,3];

    assert!(equal_to_x(y));


}
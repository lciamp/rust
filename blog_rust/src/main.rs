#![allow(unused)]

extern crate blog_rust;
use blog_rust::Post;


fn main() {

    let mut post = Post::new();

    post.add_text("I ate a salad for lunch today.");
    println!("{:?}", post);

    let post = post.request_review();
    println!("{:?}", post);

    let post = post.approve();
    println!("{:?}", post);

    assert_eq!("I ate a salad for lunch today.", post.content());

    println!("{:?}", post.content());

}
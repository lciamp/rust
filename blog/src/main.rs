#![allow(unused)]

extern crate blog;
use blog::Post;


fn main() {

    let mut post = Post::new();

    post.add_text("I ate a salad for lunch today.");

    println!("state: {}, content: {:?}", post.status(), post.content());
    assert_eq!("", post.content());

    post.request_review();

    println!("state: {}, content: {:?}", post.status(), post.content());
    assert_eq!("", post.content());

    post.approve();
    assert_eq!("I ate a salad for lunch today.", post.content());


    println!("state: {}, content: {:?}",post.status(), post.content());
}
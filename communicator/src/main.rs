#![allow(unused)]

extern crate communicator;

pub mod a {
    pub mod series {
        pub mod of {
            pub fn nested_modules(){}
        }
    }
}

enum TrafficLight {
    Red,
    Yellow,
    Green,
}

use TrafficLight::{Red, Yellow};


use a::series::of;
use a::series::of::nested_modules;

fn main() {
    let red = Red;
    let yellow = Yellow;
    let green = TrafficLight::Green;


    // why to use the 'use' key word
    a::series::of::nested_modules();

    // first use line
    of::nested_modules();

    // second use line
    nested_modules();

    communicator::client::connect();
}


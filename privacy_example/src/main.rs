pub mod a {
    pub mod series {
        pub mod of {
            pub fn nested_modules() {}
        }
    }
}

// fn main() {
//     a::series::of::nested_modules();
// }

// use a::series::of;
// 
// fn main() {
//     of::nested_modules();
// }

use a::series::of::nested_modules;

#[derive(Debug)]
enum TrafficLight {
    Red,
    Yellow,
    Green,
}

// use TrafficLight::{Red, Yellow};
use TrafficLight::*;

fn main() {
    nested_modules();

    let red = Red;
    let yellow = Yellow;
    // let green = TrafficLight::Green;
    let green = Green;

    println!("{:?}", red);
    println!("{:?}", yellow);
    println!("{:?}", green);
}

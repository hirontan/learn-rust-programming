// mod stack_heap;
// mod vars;
// mod ownership;
// mod generics;
// mod lifetime;
// mod structs;
// mod enums;
// mod traits;
// mod error_handling;
mod debug;
mod unit_test;

extern crate lib_demo;

fn main() {
    // debug::run();
    // println!("Hello, world!");
    // vars::run();
    // vars::sub_a::func_a();

    // stack_heap::run();
    // ownership::run();
    // generics::run();
    // lifetime::run();
    // structs::run();
    // enums::run();
    // traits::run();
    // error_handling::run();
    lib_demo::print_random_number();
}

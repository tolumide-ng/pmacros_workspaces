use hello_macro::hello::{ByeMacro, HelloMacro};
use hello_macro_derive::{ByeMacro, HelloMacro};

#[derive(HelloMacro, ByeMacro)]
struct Pancakes;

fn main() {
    Pancakes::hello_macro();
    println!("Even if you're not ready for the morning, it cannot always be nigth");
    Pancakes::bye_macro();
}
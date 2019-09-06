/// practice2

extern crate practice2;

use practice2::english::greetings;
use practice2::english::farewells;

fn main() {
    println!("Hello, in English: {}", greetings::hello());
    println!("Goodbye, in English: {}", farewells::goodbye());

//    println!("Hello, in English: {}", practice2::english::greetings::hello());
//    println!("Goodbye, in English: {}", practice2::english::farewells::goodbye());
//    println!("Hello, in Japanese: {}", practice2::japanese::greetings::hello());
//    println!("Goodbye, in Japanese: {}", practice2::japanese::farewells::goodbye());
}

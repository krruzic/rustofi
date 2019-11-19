// examples/simple_window.rs
use rustofi::window::*;

fn fizzbuzz() -> Vec<String> {
    let mut results = Vec::new();
    // print fizzbuzz as a rofi!
    for x in 1..25 {
        // divisible by 3 or by 5
        match (x % 3 == 0, x % 5 == 0) {
            (false, false) => results.push(x.to_string()), // neither
            (false, true) => results.push("Fizz".to_string()), // divisible by 5
            (true, false) => results.push("Buzz".to_string()), // divisible by 3
            (true, true) => results.push("FizzBuzz".to_string()), // divisible by both
        }
    }
    results
}

fn main() {
    // create a window with 8 lines and a vector of strings and show it
    Window::new("FizzBuzz in Rofi!").lines(8).show(fizzbuzz());
}
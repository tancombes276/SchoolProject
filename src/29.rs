// File: main.rs
use std::io::{self, Write};

fn main() {
    // Example of handling input and output using `std::io`
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");

    let lines = input.split_whitespace();
    for line in lines {
        println!("{}", line);
    }
}

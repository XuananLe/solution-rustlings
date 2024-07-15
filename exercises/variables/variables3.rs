// variables3.rs
//
// Execute `rustlings hint variables3` or use the `hint` watch subcommand for a
// hint.

use std::io;
fn read_input() -> i32{
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let input: i32 = input.trim().parse().unwrap();
    input
}


fn main() {
    let x: i32 = 12;
    println!("Number {}", x);
}

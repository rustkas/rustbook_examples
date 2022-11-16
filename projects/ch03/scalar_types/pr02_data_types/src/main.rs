#![allow(unused)]
fn main() {
    let guess: u32 = "42".parse().expect("Not a number!");
    let decimal = 98_222;
    let hex = 0xff;
    let octal = 0o77;
    let binary = 0b1111_0000;
    let byte = b'A';

    let x = 2.0; // f64
    let y: f32 = 3.0; // f32
}

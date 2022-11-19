#![allow(unused)]
enum OptionI32 {
    Some(i32),
    None,
}

enum OptionF64 {
    Some(f64),
    None,
}

fn main() {
    let integer = OptionI32::Some(5);
    let float = OptionF64::Some(5.0);
}

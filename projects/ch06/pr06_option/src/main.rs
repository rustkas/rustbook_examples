#![allow(unused)]

fn main() {
    let some_number = Some(5);
    let some_char = Some('e');

    let absent_number: Option<i32> = None;

    dbg!(some_number);
    dbg!(some_char);
    dbg!(absent_number);

    let x: i8 = 5;
    let y: Option<i8> = Some(5);

    // let sum = x + y;
}

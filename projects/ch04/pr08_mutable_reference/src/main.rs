#![allow(unused)]

fn main() {
    let mut s = String::from("hello");

    change(&mut s);

    println!("{s}");

    // let r1 = &mut s;
    // let r2 = &mut s;

    // println!("{}, {}", r1, r2);

    let mut s = String::from("hello");

    let r1 = &s; // no problem
    let r2 = &s; // no problem
    // let r3 = &mut s; // BIG PROBLEM
    let r3 = &s;

    println!("{}, {}, and {}", r1, r2, r3);


    let mut s = String::from("hello");

    let r1 = &s; // no problem
    let r2 = &s; // no problem
    println!("{} and {}", r1, r2);
    // variables r1 and r2 will not be used after this point

    let r3 = &mut s; // no problem
    println!("{}", r3);
}

fn change(some_string: &mut String) {
    some_string.push_str(", world");
}
fn main() {
    let s = String::from("hello");

    let slice1 = &s[0..2];
    let slice2 = &s[..2];
    println!("{slice1} {slice2}");

    println!("===================");

    let len = s.len();
    let slice1 = &s[3..len];
    let slice2 = &s[3..];
    println!("{slice1} {slice2}");

    let len = s.len();

    let slice1 = &s[0..len];
    let slice2 = &s[..];
    println!("{slice1} {slice2}");
}

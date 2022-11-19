#![allow(unused)]

fn main() {
    let mut s = String::from("hello world");
    let first = first_word(&s);
    let second = second_word(&s); 

    // s.clear();
    
    println!("First word  = {first}");
    println!("Second word = {second}");
    println!("======================");

    let word = first_word(&s[0..6]);
    let word = first_word(&s[..]);
    // `first_word` also works on references to `String`s, which are equivalent
    // to whole slices of `String`s
    let word = first_word(&s);

    let my_string_literal = "hello world";

    // `first_word` works on slices of string literals, whether partial or whole
    let word = first_word(&my_string_literal[0..6]);
    let word = first_word(&my_string_literal[..]);

    // Because string literals *are* string slices already,
    // this works too, without the slice syntax!
    let word = first_word(my_string_literal);
}

fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}

fn second_word(s: &str) -> &str {
    let bytes = s.as_bytes();
    let mut firt_index=0;
    let second_index;
    let mut first_index_found = false;
    for (i, &item) in bytes.iter().enumerate() {
        if !first_index_found && item == b' ' {
            firt_index = i+1;
            first_index_found = true;
        }else if first_index_found && item == b' ' {
            second_index = i;
            return &s[firt_index..second_index];
        }
    }
    if s.len() > 0 {
        return &s[firt_index..]; 
    }
    &s[..]
}
#![allow(unused)]

fn main() {
    let mut s = String::from("hello world");

    let word = first_word(&s); // word will get the value 5

    s.clear(); // this empties the String, making it equal to ""

    // word still has the value 5 here, but there's no more string that
    // we could meaningfully use the value 5 with. word is now totally invalid!

    println!("index of a space = {word}");
}

fn first_word(s: &String) -> usize {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return i;
        }
    }

    s.len()
}

fn second_word(s: &String) -> (usize, usize) {
    let bytes = s.as_bytes();
    let mut firt_index=0;
    let second_index;
    let mut first_index_found = false;
    for (i, &item) in bytes.iter().enumerate() {
        if !first_index_found && item == b' ' {
            firt_index = i;
            first_index_found = true;
        }else if first_index_found && item == b' ' {
            second_index = i;
            return (firt_index, second_index);
        }
    }
    if(s.len() > 0) {
        (firt_index, s.len()-1); 
    }
    (0, 0)
}
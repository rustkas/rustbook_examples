fn main() {
    let s = String::from("hello world");
    let first = first_word(&s); 
    println!("index of a space = {first}");
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
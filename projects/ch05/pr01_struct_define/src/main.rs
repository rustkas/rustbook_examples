#![allow(unused)]

struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

fn main() {
    let mut user1 = User {
        email: String::from("someone@example.com"),
        username: String::from("someusername123"),
        active: true,
        sign_in_count: 1,
    };

    user1.email = String::from("anotheremail@example.com");

    // println!("{user1.email}");
    // println!("{user1}");
    // println!("Hello, world!");

    let user2 = build_user(String::from("hello@test.com"), String::from("hello"));
    // println!("{user2.username}");
    let user3 = build_user2(String::from("hello@test.com"), String::from("hello"));
}

fn build_user(email: String, username: String) -> User {
    User {
        email: email,
        username: username,
        active: true,
        sign_in_count: 1,
    }
}

fn build_user2(email: String, username: String) -> User {
    User {
        email,
        username,
        active: true,
        sign_in_count: 1,
    }
}

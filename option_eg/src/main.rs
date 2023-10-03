// Option is an enum with two variants: Some(T) and None
// Option<T> is so useful that it's even included in the prelude; you don't need to bring it into scope explicitly.
// The Option<T> enum is still just a regular enum, and Some(T) and None are still variants of type Option<T>
// The <T> syntax is a generic type parameter. It means the Some variant of the Option enum can hold one piece of data of any type.
// The None variant doesn't hold any value, and Rust doesn't let us use a value that might be null.
// Rust doesn't have the null feature that many other languages have.
// Instead, Rust has an enum that can encode the concept of a value being present or absent.
// This enum is Option<T>, and it is defined by the standard library as follows:
// enum Option<T> {
//     Some(T),
//     None,
// }

struct Survey {
    q1: Option<i32>,
    q2: Option<String>,
    q3: Option<bool>, 
}

fn main() {
    let response = Survey {
        q1: None,
        q2: Some(String::from("Blue")),
        q3: Some(true),
    };

    match response.q1 {
        Some(answer) => println!("Answer to q1 is {}", answer),
        None => println!("No answer for q1")
    }

    match response.q2 {
        Some(answer) => println!("Answer to q2 is {}", answer),
        None => println!("No answer for q2")
    }

    match response.q3 {
        Some(answer) => println!("Answer to q3 is {}", answer),
        None => println!("No answer for q3")
    }
}

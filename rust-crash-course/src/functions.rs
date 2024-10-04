#![deny(clippy::all)]

pub fn say_hello_world(message: &str) -> String {
    // let message = String::from("Hello, world");
    format!("Hello, {}!", message)
}



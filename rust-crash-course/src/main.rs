#![deny(clippy::all)]

fn main() {
    println!("Hello, world!");

    let first_name: &str = "Foo";

    println!("Hello {}!", first_name);

    let testing: &str = "testing rust compiler";
}

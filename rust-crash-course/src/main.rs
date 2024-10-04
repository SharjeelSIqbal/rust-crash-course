#![deny(clippy::all)]

//* Constants variables than can never change */
const MY_AGE: u8 = 22;

// mod ownership;
mod functions;

fn _process_name(name: &str, callback: fn(&str) -> ()) {
    callback(name);
}

fn main() {
    let _say_hello_to = |name: &str| format!("Hello {}", name);
    let _full_name = |first_name: &str, last_name: &str| format!("{} {}", first_name, last_name);
    let _multiplied_by_two = |num: i32| num * 2;
    let _ask_for_age = || println!("Something");

    let ptr = _multiplied_by_two;
    let _result = ptr(10);

    fn str_function(name: &str) -> String {
        format!("Hello {}", name)
    }
    // Returns a String
    let _greeting = str_function("Sharjeel");

    // println!("{}", greeting);

    fn _void_function(name: &str) {
        println!("Hello my name is {}", name);
    }
    // returns nothing

    // void_function("Sharjeel");
    // Prints "Hello my name is Sharjeel

    //* Constants Cont... */
    let _age_constant = MY_AGE;
    // Data Types

    //* Integers iX uX*/
    // Signed values can be negative or positive, most use cases are i32
    let _age: i32 = -20;

    // Unsigned values cannot be negative
    let _unsigned: u8 = 20;

    let _unsigned_data_type = 20u8;

    let _population = 62000000;
    let _population_underscore = 62_000_000;

    let _red = 0xFA;
    let _rgb = 0xFF0000;

    //* Floating Point Values f32 f64 */
    let _floating: f32 = 5.5;

    let _distance_in_km = 5.5f32;

    //* Operators */
    let distance1 = 5.5;
    let distance2 = 6.5;
    let distance3 = 10.2;

    let _total_distance = distance1 + distance2 + distance3;

    //* Shadowing */
    let _data: &str = "Foo";

    let _data = "hello";

    // Shadowing allows you to change variable values other wise they are immutable
    // You can change the data type as well

    let _data = 32;

    let _data = "Foo";
    {
        let _data = _data.to_string();
    }

    //* Tuples */
    // Unlike objects they're not key value pairs
    let personal_data = (22u8, "John");

    // Grabbing data
    let (_age_tuple, _name_tuple) = personal_data;

    let _age_tuple_direct = personal_data.0;

    let _name_tuple_direct = personal_data.1;

    // You can iterate over tuples as well. Usually done in collections

    // ownership::ownership();
    let hello = functions::say_hello_world("Hello World!");

    println!("{}", hello);
}

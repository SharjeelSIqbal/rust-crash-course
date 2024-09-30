#![deny(clippy::all)]

//* Constants variables than can never change */
const MY_AGE: u8 = 22;

mod ownership;

fn main() {
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
    let (_age_tupple, _name_tupple) = personal_data;

    let _age_tupple_direct = personal_data.0;

    let _name_tupple_direct = personal_data.1;

    // You can iterate over tuples as well. Usually done in collections

    ownership::ownership();
}

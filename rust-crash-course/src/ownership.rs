fn greet(name: &String) {


  /* ----- Ownership Rules -----
    1. Each value is Rust has a variable that's called its owner
    2. There can only be one owner at a time
    3. When the owner goes out of scope, the value is dropped

   */
    println!("Hello {} pointer", name);
}

fn empty_string(value: &mut String) {
    println!("Clears string {}", value);
    value.clear();
}

pub fn ownership() {
    //* Ownership */
    println!("Hello!");

    // Transferring Ownership

    let name1 = String::from("John");
    println!("Hello {}", name1);
    let name2 = name1;

    // println!("Hello {}", name1);
    // The line above results in a compilation error
    // After name1 has been assigned to name2 and no longer exists
    // Rust dumps it for garbage collection and memory management
    // This example is for Strings

    println!("Hello {}", name2);

    let _a = name2;

    {
        let name = "John".to_string();
        println!("Hello, {}", name);
    }

    // Rust drops variables that go out of scope

    // For integers it works just fine

    {
        let age1 = 10;
        let age2 = age1;
        println!("You are {} years old", age1);
        println!("You are {} years old", age2);
    }

    // * Stack vs. Heap */
    /*  Think of Stacks as pringles
      When you add to a "stack" you add a chip on top of the other
      If you want to reach the bottom you have to "pop" off the top stack
      Similar to pringles
      for small short lived variables (integers, structs, function calls)
      Stacks can only exist within that stacks in a multi threaded application
      there is a Stack in each thread

    */

    /*
      Heap space
      Random Access, you can put memory any where like a regular bag of chips
      but unlike a bag of chips you need to put the chip back or release the data for
      garbage collection
      Heaps are shared amongst a multi-threaded application which can cause memory leaks if
      memory isn't free'd
      For more complex data (Large arrays, objects, or dynamically sized data)
    */

    /*
      References
      A pointed to a variable, references are written with an ampersand
      s1 is a stack that points a value in the heap
      where as s2 is a reference and points to the variable s1
      s2 is read only


      references don't drop their value
    */

    {
        let s1 = String::from("John");
        let _s2 = &s1;
        greet(_s2);
    }

    {
        let mut name = String::from("John");
        let name2 = &mut name;

        // You cannot have more than one mutable reference to a variables
        // let name3 = &mut name;
        empty_string(name2);

        println!("Cleared string {}", name2);
        println!("Uncleared String {}", name);
        // empty_string(name3);
    }


    // No mutable references while immutable references are there
    {
      let mut name1 = String::from("John");
      // let name2 = &mut name1;

      // let name3 = &mut name1;

      println!("{}", name1);
      // println!("{}", name2);
      // println!("{}", name3);
    }

    // Dangling references
    // Since this is returning a reference that exists within the stack and the heap it's wondering
    // how you're going to get rid of it?
    // fn get_name () -> &String {
    //   &"John".to_string()
    // }
}

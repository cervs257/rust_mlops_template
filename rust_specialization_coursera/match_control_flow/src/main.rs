// match control flow
// match is a control flow operator like if/else

// basic match usage
// fn main() {
//     let name = "Hello";

//     // use of match expression to pattern match agains variable name
//     match name {
//         "Hello" => println!("Hello Indeed!"),
//         "Goodbye" => println!("Goodbye World!"),
//         // the following is used to catch anything else
//         _ => println!("I don't know what you mean!"),
//     }
// }

// slightly more complex match usage
use std::io;

fn main() {
    println!("Please enter a greeting: ");
    let mut greeting = String::new();
    // read the greeting and handle any errors
    io::stdin()
        .read_line(&mut greeting)
        .expect("Failed to read line");

    // use of match expression to pattern match agains variable greeting
    match greeting.to_lowercase().trim() {
        "hello" => println!("Hello Indeed!"),
        "goodbye" => println!("Goodbye World!"),
        // the following is used to catch anything else
        _ => println!("I don't know what you mean!"),
    }
}

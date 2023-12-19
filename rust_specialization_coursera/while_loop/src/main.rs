// a more advanced type of while loop that allows it to continue, asking for user input until the user types a specific word

use std::io;

fn main() {
    // create a new empty string ... equivalent to let user_input = "";
    let mut user_input = String::new();

    // .trim() removes any whitespace from the beginning or end of the string
    while user_input.trim() != "quit" {
        println!("Enter a word, or type 'quit' to exit: ");
        user_input.clear();
        // now we can use .read_line() to read the user input and store it in the user_input variable
        io::stdin()
            .read_line(&mut user_input)
            .expect("Failed to read line");
        println!("You entered: {}", user_input);
    }
}

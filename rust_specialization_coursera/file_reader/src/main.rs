use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader};

// this project reads a file and prints its content to the console
// there are two options to pass a file path to the program:
// 1) as a command-line argument
// 2) if no argument passes in command-line, program asks for user input

fn main() {
    // two options: pass file_path as a command-line argument or ask user to input a file path:
    // path 1: command-line argument
    let args: Vec<String> = env::args().collect();
    let file_path = if args.len() > 1 {
        args[1].clone()
    } else {
        // path 2: program asks user to input file_path
        let mut file_path = String::new();
        // as user to input a file path
        println!("Please enter a file path:");
        std::io::stdin()
            // read_line needs a mutable reference because read_line appends the user input to the string
            .read_line(&mut file_path)
            .expect("Failed to read line");
        // read_line appends (\n) to the string, so we need to trim it
        // use .to_string() otherwise it will return &str and won't compile because file_path in the if statement is a String
        let file_path = file_path.trim().to_string();
        file_path
    };

    let file = File::open(file_path);
    let file = match file {
        Ok(file) => file,
        Err(error) => match error.kind() {
            std::io::ErrorKind::NotFound => {
                panic!("File not found: {}", error)
            }
            _ => {
                panic!("Error opening file: {}", error)
            }
        },
    };

    let reader = BufReader::new(file);
    for line in reader.lines() {
        match line {
            Ok(line) => println!("{}", line),
            Err(error) => {
                panic!("Error reading line: {}", error)
            }
        }
    }
}

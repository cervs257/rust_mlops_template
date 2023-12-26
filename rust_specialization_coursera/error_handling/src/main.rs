use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    // define file
    let file = File::open("non_existent_file.txt");
    // use match to handle error
    let file = match file {
        Ok(file) => file, // if file exists, return file
        Err(error) => {
            // we want to handle depending on the type of error
            match error.kind() {
                // not found error
                std::io::ErrorKind::NotFound => {
                    panic!("File not found: {}", error)
                }
                // all other errors
                _ => {
                    panic!("Error opening file: {}", error)
                }
            }
        }
    };

    // if we get here, file exists
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

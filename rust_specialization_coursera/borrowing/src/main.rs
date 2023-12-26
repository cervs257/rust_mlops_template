fn own_vec(mut vector: Vec<i32>) {
    vector.push(10);
    println!("Own {:?}", vector);
}

fn borrow_vec(vector: &Vec<i32>) {
    // vector.push(10); // this will not compile because the function borrow_vec borrowed vector
    // a workaround is to create a new vector
    let mut new_vector = vector.clone();
    new_vector.push(10);
    println!("New vector {:?}", new_vector);
}

fn own_integer(x: i32) {
    x + 1; // will throw a warning because this is not used
    println!("x+1 = {}", x + 1) // x is immutable so x won't change
}

// defining the function as this takes ownership of s:
fn own_string(s: String) {
    println!("Own {}", s);
}

// defining the function as this borrows s and won't cause issues:
fn borrow_string(s: &String) {
    println!("Borrow {}", s);
}

// Borrowing is the mechanism by which Rust allows you to lend ownership of a variable to a function
// or another part of your program without actually transferring ownership of the variable.
// When you borrow a variable, you're essentially saying
// "I want to use this variable for a little while, but I promise I won't modify it."
fn main() {
    let my_vec = vec![1, 2, 3, 4, 5];
    let my_int = 10;
    let my_string = String::from("Hello, world!");

    // this compiles no problem!
    own_integer(my_int);
    println!("x after function {}", my_int);

    own_string(my_string); // take ownership of my_string
                           // this is using my_string which has also moved and is invalid
                           // println!("{:?}", my_string); // this will not compile because the function own_string took ownership of my_string!

    let my_string = String::from("Hello, world!");
    borrow_string(&my_string);
    println!("Original {}", my_string); // this will compile because the function borrow_string borrowed my_string

    own_vec(my_vec); // fn takes ownership of my_vec and redefines it as mutable
                     // but this is using my_vec which was borrowed (moved) and yet is now invalid
                     // println!("{:?}", my_vec); // this will not compile!

    let my_vec = vec![1, 2, 3, 4, 5];
    borrow_vec(&my_vec);
    println!("Original vector {:?}", my_vec); // this will compile because the function borrow_vec borrowed my_vec
}

// Borrowing is a key concept in Rust because it allows you to write code that is both safe and efficient.
// By lending ownership of a variable instead of transferring it, Rust ensures that only
// one part of your program can modify the variable at a time, which helps prevent
// bugs and makes it easier to reason about your code.

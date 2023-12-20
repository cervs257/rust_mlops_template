// different ways to use for loop

fn main() {
    // for loop with range
    for i in 0..5 {
        println!("for loop with range: {}", i);
    }

    // same for loop but with 5 inclusive
    for i in 0..=5 {
        println!("for loop with range: {}", i);
    }

    // same for loop but implementing continue
    for i in 0..=5 {
        if i == 3 {
            continue; // this continue is equivalent to pass in Python .. in this example, it won't print 3
        }
        println!("for loop with range: {}", i);
    }

    // iterating over arrays and vectors:
    // there's a difference between referencing the array/vector and moving it into the for loop
    // referencing it allows you to use the array/vector after the loop
    // moving it into the loop does not allow you to use the array/vector after the loop
    // moving it into the loop is faster than referencing it

    // for loop with array
    let arr = [1, 2, 3, 4, 5];
    for i in arr.iter() {
        println!("for loop with array: {}", i);
    }

    // for loop with vector where the vector can still be used after the loop
    let vec = vec![1, 2, 3, 4, 5];
    for i in vec.iter() {
        println!("for loop with vector: {}", i);
    }

    // for loop with vector where the vector cannot be used after the loop
    let vec = vec![1, 2, 3, 4, 5];
    for i in vec {
        println!("for loop with vector: {}", i);
    }

    // for loop with string
    let str = "Hello World!";
    for i in str.chars() {
        println!("for loop with string: {}", i);
    }

    // for loop with string
    let str = "Hello World!";
    for i in str.bytes() {
        println!("for loop with string: {}", i);
    }

    // In Rust, tuples are not iterable with a for loop because they can contain elements of different types

    // trick for creating a range for loop using the first element of a tuple and the fourth element of a tuple
    let tup = (4, 2, 7, 14, 56);
    for i in tup.0..tup.4 {
        println!("for loop with tuple: {}", i);
    }
}

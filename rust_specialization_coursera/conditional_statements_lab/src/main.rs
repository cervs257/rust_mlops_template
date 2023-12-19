// There are other conditionals that we can explore in Rust. Like using `if let`

// fn main() {
//     // let maybe_number: Option<Option<()>> = None; // allows maybe_number to be None or Some(T)
//     let maybe_number = Some(42);
//     // if let is equivalent to asking if maybe_number is not None
//     if let Some(number) = maybe_number {
//         // note that maybe_number gets assigned to number
//         println!("The number is {:?}", number);
//     } else {
//         println!("There is no number");
//     }
// }

// Challenge questions
fn main() {
    let maybe_number = Some(24);

    if let Some(number) = maybe_number {
        if number == 42 {
            println!("The number is the answer to life, the universe, and everything!");
        } else {
            println!("The number is {:?}", number);
        }
    } else {
        println!("There is no number");
    }
}

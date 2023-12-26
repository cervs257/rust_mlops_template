// // Declare Car struct to describe vehicle with four named fields
// struct Car {
//     color: String,
//     transmission: Transmission,
//     convertible: bool,
//     mileage: u32,
// }

// #[derive(PartialEq, Debug)]
// // Declare enum for Car transmission type
// enum Transmission {
//     // todo!("Fix enum definition so code compiles");
//     Manual,
//     SemiAuto,
//     Automatic,
// }

// fn car_factory(color: String, transmission: Transmission, convertible: bool) -> Car {
//     Car {
//         color: color,
//         transmission: transmission,
//         convertible: convertible,
//         mileage: 0,
//     }
// }

// fn main() {
//     let mut car = car_factory(String::from("Red"), Transmission::Manual, false);
//     println!(
//         "Car 1: {}, {:?} transmission, convertible: {}, mileage: {}",
//         car.color, car.transmission, car.convertible, car.mileage
//     );

//     car = car_factory(String::from("Silver"), Transmission::Automatic, true);
//     println!(
//         "Car 2 = {}, {:?} transmission, convertible: {}, mileage: {}",
//         car.color, car.transmission, car.convertible, car.mileage
//     );

//     car = car_factory(String::from("Yellow"), Transmission::SemiAuto, false);
//     println!(
//         "Car 3 = {}, {:?} transmission, convertible: {}, mileage: {}",
//         car.color, car.transmission, car.convertible, car.mileage
//     );
// }

fn sum(numbers: &[i32]) -> i32 {
    let mut result = 0;
    for number in numbers {
        result += number;
    }
    result
}

fn main() {
    // let the user input a series of numbers. When the user types "done", print out the sum of the numbers.
    let mut numbers = Vec::new();
    loop {
        let mut input = String::new();
        std::io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");
        let input = input.trim();
        if input == "done" {
            break;
        }
        let number: i32 = match input.parse() {
            Ok(n) => n,
            Err(_) => continue,
        };
        numbers.push(number);
    }

    let result = sum(&numbers);
    println!("The sum is {}", result);
}

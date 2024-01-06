#[derive(Debug)]
enum Shape {
    Circle(f64),
    Square(f64),
    Triangle(f64, f64),
}

fn main() {
    let shapes = vec![
        Shape::Circle(5.0),
        Shape::Square(3.0),
        Shape::Triangle(2.0, 6.0),
    ];

    let total_area: f64 = shapes
        .iter()
        .map(|shape| match shape {
            Shape::Circle(radius) => std::f64::consts::PI * radius * radius,
            Shape::Square(length) => length * length,
            Shape::Triangle(base, height) => base * height / 2.0,
        })
        .sum();

    let largest_shape = shapes
        .iter()
        .map(|shape| {
            let area = match shape {
                Shape::Circle(radius) => std::f64::consts::PI * radius * radius,
                Shape::Square(length) => length * length,
                Shape::Triangle(base, height) => base * height / 2.0,
            };
            (shape, area)
        })
        .max_by(|(_, a), (_, b)| a.partial_cmp(b).unwrap())
        .unwrap();

    println!(
        "The largest shape is {:?} with an area of {}",
        largest_shape.0, largest_shape.1
    );

    println!("Total area: {} sq. units", total_area);
}

// .map applies a function to each item in an iterator and produces a new iterator with the results.
// in this example .map is called on an iterator over the shapes vector.
// The function passed to .map is a closure (an anonymous function) that takes a reference to a Shape (shape) and returns the area of the shape.

// in the largest_shape variable, .map is called on an iterator over the shapes vector.
// inside the iterator, area is calculated for each shape and the shape and area are returned as a tuple.
// .max_by is called on the iterator to find the largest shape by comparing the areas.

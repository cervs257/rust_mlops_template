// Like structs, enum variants can have named fields, but they can also have fields without names, or no fields at all.
// Like struct types, enum types are also capitalized.

// define an enum to classify a web event. Each variant in the enum is independent and stores different amounts and types of values.
// NOTE Each variant in the enum isn't its own type. Any function that uses a variant of the WebEvent enum must accept all the variants in the enum
// enum WebEvent {
//     // An enum variant can be like a unit struct without fields or data types
//     WELoad,
//     // An enum variant can be like a tuple struct with data types but no named fields
//     WEKeys(String, char),
//     // An enum variant can be like a classic struct with named fields and their data types
//     WEClick { x: i64, y: i64 },
// }

// #[derive(Debug)] syntax lets us see certain values during the code execution that aren't otherwise viewable in standard output

// A way to work around enum variant requirements is to define a separate struct for each variant in the enum.
// Define a tuple struct
#[derive(Debug)] // allows the struct to be printed using the {:?} or {:#?} placeholder
struct KeyPress(String, char);

// Define a classic struct
#[derive(Debug)]
struct MouseClick {
    x: i64,
    y: i64,
}

// Redefine the enum variants to use the data from the new structs
// Update the page Load variant to have the boolean type
#[derive(Debug)]
enum WebEvent {
    WELoad(bool),
    WEClick(MouseClick),
    WEKeys(KeyPress),
}

fn main() {
    // Instantiate a MouseClick struct and bind the coordinate values
    let click = MouseClick { x: 100, y: 250 };
    println!("Mouse click location: {}, {}", click.x, click.y);

    // Instantiate a KeyPress tuple and bind the key values
    let keys = KeyPress(String::from("Ctrl+"), 'N');
    println!("\nKeys pressed: {}{}", keys.0, keys.1);

    // Instantiate WebEvent enum variants
    // Set the boolean page Load value to true
    let we_load = WebEvent::WELoad(true);
    // Set the WEClick variant to use the data in the click struct
    let we_click = WebEvent::WEClick(click);
    // Set the WEKeys variant to use the data in the keys tuple
    let we_key = WebEvent::WEKeys(keys);

    // Print the values in the WebEvent enum variants
    // Use the {:#?} syntax to display the enum structure and data in a readable form
    println!(
        "\nWebEvent enum structure: \n\n {:#?} \n\n {:#?} \n\n {:#?}",
        we_load, we_click, we_key
    );
}

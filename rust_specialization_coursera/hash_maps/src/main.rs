use std::collections::HashMap;

fn main() {
    // create a new empty hash map which will take string, string as key, value pairs
    let mut reviews: HashMap<String, String> = HashMap::new();

    // insert key, value pairs into the hash map. Note the from() method
    // the from function is used to create a String object from a string literal
    // This is necessary because the insert method of a HashMap takes ownership of the key and value.
    // String literals (&str) are borrowed types and cannot be moved into the HashMap, so they need to be converted to owned String objects.
    reviews.insert(
        String::from("Ancient Roman History"),
        String::from("Very accurate."),
    );
    reviews.insert(
        String::from("Cooking with Rhubarb"),
        String::from("Sweet recipes."),
    );
    reviews.insert(
        String::from("Programming in Rust"),
        String::from("Great examples."),
    );

    // Look for a specific review
    // note the use of &str instead of String.
    // This is more efficient because using STring would require the string literal to be copied into a newly allocated String object
    let book: &str = "Programming in Rust";
    // note the use of {:?} which tells Rust to use the Debug trait to format the value
    println!("\nReview for \'{}\': {:?}", book, reviews.get(book));

    // Remove book review
    let obsolete: &str = "Ancient Roman History";
    println!("\n'{}\' removed.", obsolete);
    reviews.remove(obsolete);

    // Confirm book review removed
    println!("\nReview for \'{}\': {:?}", obsolete, reviews.get(obsolete));
}

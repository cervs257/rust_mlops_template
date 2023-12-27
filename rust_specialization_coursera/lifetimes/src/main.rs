// so Rust knows that the lifetime of the return value is the same as the lifetime of the vector
fn copy_and_return<'lifetime>(
    vector: &'lifetime mut Vec<String>,
    value: &'lifetime str,
) -> &'lifetime String {
    vector.push(String::from(value));
    vector.get(vector.len() - 1).unwrap() // using .get(index) instead of [index] to avoid panic in case of out of bounds
}

fn main() {
    let name1 = "Joe";
    let name2 = "Chris";
    let name3 = "Anne";

    let mut names = Vec::new();

    assert_eq!("Joe", copy_and_return(&mut names, &name1));
    assert_eq!("Chris", copy_and_return(&mut names, &name2));
    assert_eq!("Anne", copy_and_return(&mut names, &name3));

    assert_eq!(
        names,
        vec!["Joe".to_string(), "Chris".to_string(), "Anne".to_string()]
    )
}

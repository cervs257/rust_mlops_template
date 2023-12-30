// There may be multiple lifetimes. When that occurs,
// annotate the lifetimes to help the compiler understand which lifetime it will use to ensure the references are valid at runtime.
// The borrow checker can't determine if the reference will be a valid one either.
// It doesn't know how the input parameters' lifetime relates to the return value's lifetime.
//This ambiguity is why we need to annotate the lifetimes manually.

// Rust doesn't know if the return value will be a reference to the vector or a reference to the value parameter => annotate lifetimes manually
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

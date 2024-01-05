fn main() {
    let mut v = vec![1, 2, 3];
    v.push(4);
    //println!("{:?}", v); // Output: [1, 2, 3, 4]

    // extend uses an iterator on more_numbers and adds each element of the given slice to the vector
    // more_numbers is left intact and can be used again
    let more_numbers = vec![5, 6];
    v.extend(more_numbers);
    //println!("{:?}", v);

    // append moves other_numbers to the vector v, requires the vector to be mutable because all of its elements are moved into v
    // unlike when using extend, this means other_numbers will no longer be able to be used
    let mut other_numbers = vec![7, 8];
    v.append(&mut other_numbers);
    println!("{:?}", v);
    // the following vector should be empty
    println!("{:?}", other_numbers);

    // insert items at a given index
    v.insert(0, 0);
    println!("{:?}", v); // Output: [0, 1, 2, 3, 4, 5, 6, 7, 8]
}

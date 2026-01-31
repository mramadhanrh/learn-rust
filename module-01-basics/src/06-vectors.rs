fn main() {
    let mut vec = vec![1, 2, 3];

    // Push only allow 1 element
    vec.push(4);
    println!("Vectors contents {:?}", vec);

    assert_eq!(vec.len(), 4);

    // Extend allow pushing multiple elements
    vec.extend([1, 2, 3]);
    println!("Vectors contents {:?}", vec);

    // This is like for each in javascript
    for x in vec {
        println!("{x}")
    }
}

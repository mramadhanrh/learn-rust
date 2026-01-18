fn immutable_variable() {
    let x = 5;
    println!("The value of x: {x}");
    // x = 6; This will cause an error because x is immutable
}

fn mutable_variable() {
    let mut y = 10;
    println!("The initial value of y: {y}");
    y = 15; // This is allowed because y is mutable
    println!("The updated value of y: {y}");
}

// Variables and Mutability
fn main() {
    immutable_variable();

    mutable_variable();
}
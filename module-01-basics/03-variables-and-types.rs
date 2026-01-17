fn main() {
    // Integer variable
    let int32: i32 = 42;
    let int64: i64 = 42000000000;

    println!("Integer int32: {}", int32);
    println!("Integer int64: {}", int64);

    // Floating-point variable
    let float32: f32 = 3.14;
    let float64: f64 = 2.718281828459045;
    println!("Floating-point float32: {}", float32);
    println!("Floating-point float64: {}", float64);

    // Boolean variable
    let is_rust_fun: bool = true;
    println!("Boolean is_rust_fun: {}", is_rust_fun);

    let is_rust_boring: bool = false;
    println!("Boolean is_rust_boring: {}", is_rust_boring);

    // No need parentheses for if statements in Rust
    if is_rust_fun && !is_rust_boring {
        println!("Rust is fun!");
    } else {
        println!("Rust is boring.");
    }

    // Character variable
    let letter: char = 'R';
    let emoji: char = '😊';
    println!("Character letter: {}", letter);
    println!("Character emoji: {}", emoji);
}

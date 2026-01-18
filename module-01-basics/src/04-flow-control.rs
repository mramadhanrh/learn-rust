// Dead code allowance for the enum since it may not be used in all contexts
#[allow(dead_code)]
enum Direction {
    North,
    South,
    East,
    West,
}

fn main() {
    // Example inline variable declaration and if-else flow control
    let age = 58;
    let status = if age > 50 {
        "You are old."
    } else {
        "You are young."
    };

    println!("{}", status);

    // Another example with direct if-else without variable assignment
    let my_new_age = 28;

    if my_new_age > 50 {
        println!("You are old.");
    }

    // Matching
    let num = 100;

    match num {
        100 => println!("The number is 100"),
        200 => println!("The number is 200"),
        _ => println!("Something else"),
    }

    // Using match with ranges
    let score = 85;
    match score {
        0..=59 => println!("Failing grade"),
        60..=69 => println!("D grade"),
        70..=79 => println!("C grade"),
        80..=89 => println!("B grade"),
        90..=100 => println!("A grade"),
        _ => println!("Invalid score"),
    }

    // Using match with enums
    let dir = Direction::East;
    match dir {
        Direction::North => println!("Heading North"),
        Direction::South => println!("Heading South"),
        Direction::East => println!("Heading East"),
        Direction::West => println!("Heading West"),
    }

    // Loops Example
    let mut count = 0;
    while count < 5 {
        println!("While Count is: {}", count);
        count += 1;
    }

    for i in 0..5 {
        println!("For loop iteration: {}", i);
    }
}

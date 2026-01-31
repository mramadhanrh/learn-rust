fn get_reference_value() {
    let a = 42;
    let b = &a;

    println!("{b}");

    let mut c = *b;
    c += 1;

    println!("{c}");

    let s1 = String::from("hello");
    let s2 = s1;

    // println!("{s1}"); // This will error since s1 already moved to s2, All expensive types that uses heap, like string, vec, array will be moved when reassigned to other params, while number will be copied
    println!("{s2}") // This is works
}

fn get_fibonacci_sequence() {
    let mut fibonacci_sequence: Vec<i128> = vec![];
    let length: i128 = 100;

    for x in 0..length {
        if fibonacci_sequence.len() <= 1 {
            fibonacci_sequence.push(x);
            let last_item = fibonacci_sequence.last();
            println!("Item Number {x}: {:?}", last_item);
            continue;
        }

        let len = fibonacci_sequence.len();
        let last = fibonacci_sequence[len - 1];
        let second_last = fibonacci_sequence.get(len - 2).unwrap_or(&1);

        fibonacci_sequence.push(last + second_last);

        let last_item = fibonacci_sequence.last();
        println!("Item Number {x}: {:?}", last_item);
    }
}

fn main() {
    get_fibonacci_sequence();
    get_reference_value();
}

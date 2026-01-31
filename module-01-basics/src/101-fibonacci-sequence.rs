fn main() {
    let mut fibonacci_sequence: Vec<i128> = vec![];
    let length: i128 = 100;

    for x in 0..length {
        if fibonacci_sequence.last() == Some(&1) {
            println!("why");
        }

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

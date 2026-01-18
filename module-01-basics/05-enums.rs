use std::num::ParseIntError;

enum HouseLocation {
    Number(i32),
    Name(String),
    Unknown,
}

fn match_location(loc: HouseLocation) {
    match loc {
        HouseLocation::Number(num) => println!("House number: {}", num),
        HouseLocation::Name(name) => println!("House name: {}", name),
        HouseLocation::Unknown => println!("Unknown house location"),
    }
}

fn match_option(opt: Option<i32>) {
    match opt {
        Some(value) => println!("Option has value: {}", value),
        None => println!("Option is None"),
    }
}

fn match_result(res: Result<i32, ParseIntError>) {
    match res {
        Ok(value) => println!("Result is Ok with value: {}", value),
        Err(err) => println!("Result is Err with error: {}", err),
    }
}

fn main() {
    let loc1 = HouseLocation::Number(123);
    match_location(loc1);

    let loc2 = HouseLocation::Name(String::from("Sunset Villa"));
    match_location(loc2);

    let loc3 = HouseLocation::Unknown;
    match_location(loc3);

    let opt1 = Some::<i32>(42);
    match_option(opt1);

    let opt2: Option<i32> = None;
    match_option(opt2);

    let res1 = Ok::<i32, std::num::ParseIntError>(100);
    match_result(res1);

    let res2 = i32::from_str_radix("wibble", 16);
    match res2 {
        Ok(value) => println!("Direct Result is Ok with value: {}", value),
        Err(err) => println!("Direct Result is Err with error: {}", err),
    }
}

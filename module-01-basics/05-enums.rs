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

fn main() {
    let loc1 = HouseLocation::Number(123);
    match_location(loc1);

    let loc2 = HouseLocation::Name(String::from("Sunset Villa"));
    match_location(loc2);

    let loc3 = HouseLocation::Unknown;
    match_location(loc3);
}

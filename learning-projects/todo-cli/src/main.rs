mod activity_list;

use activity_list::get_activity_list;

use crate::activity_list::ActivityList;

fn greetings() {
    println!("Hello, welcome to TODO CLI App!");
    println!("Today I want to learn Rust programming language.");
}

fn main() {
    greetings();

    let wrong_activity: Result<ActivityList, String> = Ok(ActivityList::Swimming);
    get_activity_list(wrong_activity);
}

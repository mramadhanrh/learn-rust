pub enum ActivityList {
    Hiking,
    Running,
    Swimming,
}

fn get_hiking_activity() {
    println!("You goes to mount Himalaya to Hike!");
    println!("You enjoys the beautiful scenery and fresh air while hiking.");
}

fn get_swimming_activity() {
    println!("You selected Swimming as your activity!");
}

fn get_running_activity() {
    println!("You selected Running as your activity!");
}

pub fn get_activity_list(activity: Result<ActivityList, String>) {
    match activity {
        Ok(ActivityList::Hiking) => get_hiking_activity(),
        Ok(_) => println!("Activity not implemented yet."),
        Err(err) => println!("Error: {:?}", err),
    }
}

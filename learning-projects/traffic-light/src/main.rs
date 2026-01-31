use std::{i32, thread, time};

mod traffic_light;

fn main() {
    println!("Hello, world!");

    let mut light_action: &str = traffic_light::get_light_action(traffic_light::Light::Red);
    println!("Traffic light action: {}", light_action);
    std::thread::sleep(std::time::Duration::from_secs(1));
    light_action = traffic_light::get_light_action(traffic_light::Light::Yellow);
    println!("Traffic light action: {}", light_action);
    std::thread::sleep(std::time::Duration::from_secs(1));
    light_action = traffic_light::get_light_action(traffic_light::Light::Green);
    println!("Traffic light action: {}", light_action);
}

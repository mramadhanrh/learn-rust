pub enum Light {
    Red,
    Yellow,
    Green,
}

// Notes: Quick Decision Tree
// Returning:

// Fixed text? → &'static str
// Slice of input? → &str with lifetime
// New/modified text? → String
// Parameters:

// Always use &str (most flexible)
pub fn get_light_action(light: Light) -> &'static str {
    match light {
        Light::Red => "Stop",
        Light::Yellow => "Caution",
        Light::Green => "Go",
    }
}

pub fn get_first_word<'a>(text: &'a str) -> &'a str {
    text.split_whitespace().next().unwrap_or("")
}

use std::fs;
use toml;

fn main() {
    let config = fs::read_to_string("config/user.toml").expect("Failed to read config/user.toml");
    let parsed: toml::Value = toml::from_str(&config).expect("Failed to parse toml");
    println!("{:?}", parsed);
}

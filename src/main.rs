use std::process;

use mini_programming::core;
use serde::{Deserialize, Serialize};
use serde_json::Result;
#[derive(Serialize, Deserialize)]
struct Person {
    name: String,
    age: u8,
    phones: Vec<String>,
}
fn main() {
    match core::init() {
        Ok(()) => {}
        Err(e) => {
            println!("error {}", e);
            process::exit(1);
        }
    }
}
fn parse(string_data: &str) -> Result<()> {
    let person: Person = serde_json::from_str(string_data)?;
    println!("name {} age {}", person.name, person.age);
    Ok(())
}

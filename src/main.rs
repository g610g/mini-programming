use mini_programming::{core, utils};
use serde::{Deserialize, Serialize};
use serde_json::Result;
#[derive(Serialize, Deserialize)]
struct Person {
    name: String,
    age: u8,
    phones: Vec<String>,
}
fn main() {
    foo("test");
}
fn foo(_my_str: &str) {
    let white_space_test: String = String::from("print()");
    let file_data =
        std::fs::read_to_string("assets/data.json").expect("Failed to retrieve the file");
    let data = r#" 
            {
            "name": "John Doe",
            "age": 43,
            "phones": [
                "+44 1234567",
                "+44 2345678"]
            }"#;
    println!("{}", data);
    let _ = parse(&file_data).unwrap();
}

fn parse(string_data: &str) -> Result<()> {
    let person: Person = serde_json::from_str(string_data)?;
    println!("name {} age {}", person.name, person.age);
    Ok(())
}

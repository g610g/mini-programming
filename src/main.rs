use mini_programming::core;
use std::process;
fn main() {
    match core::start("file.txt") {
        Ok(()) => {}
        Err(e) => {
            println!("Error: {}", e);
            process::exit(1);
        }
    }
}

use std::process;

use mini_programming::core;
fn main() {
    match core::start("file.txt") {
        Ok(()) => {}
        Err(e) => {
            println!("Error: {}", e);
            process::exit(1);
        }
    }
}

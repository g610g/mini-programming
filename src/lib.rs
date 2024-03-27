pub mod utils {
    //clear whitespaces of the string parameter and returns a new string
    pub fn clear_whitespaces(s: &str) -> String {
        s.chars().filter(|c| !c.is_whitespace()).collect()
    }
}
pub mod core {
    use std::collections::HashMap;

    enum Syntax {
        Print(char),
        Operation(char),
    }
    struct State {
        table_state: HashMap<String, String>,
        traversed_state: Vec<String>,
        final_state: String,
    }
    pub fn run(filtered_string: &str) {}
    //the funtion that will check if the syntax for print is valid or not
    pub fn print(s: &str) {
        let mut current_state_char: &char = &s.chars().nth(0).unwrap();
        println!("{current_state_char}");
        // for character in s.chars(){}
    }
    pub fn init_states() {}
}

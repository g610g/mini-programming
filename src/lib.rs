pub mod utils {
    //clear whitespaces of the string parameter and returns a new string
    pub fn clear_whitespaces(s: &str) -> String {
        s.chars().filter(|c| !c.is_whitespace()).collect()
    }
}
pub mod core {
    enum Syntax {
        Print(char),
        Operation(char),
    }
    pub fn run(filtered_string: &str) {}
    pub fn print(s: &str) {}
}

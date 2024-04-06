pub mod utils {
    //clear whitespaces of the string parameter and returns a new string
    pub fn clear_whitespaces(s: &str) -> String {
        s.chars().filter(|c| !c.is_whitespace()).collect()
    }
}
pub mod core {
    use std::{collections::HashMap, error::Error, fs};

    use serde::{Deserialize, Serialize};
    struct Syntax {}
    #[derive(Serialize, Deserialize, Debug)]
    struct Print {
        states: HashMap<String, HashMap<String, String>>,
    }
    pub fn init() -> Result<(), Box<dyn Error>> {
        let syntax_path = "assets/syntax.json";
        let syntax_string = fs::read_to_string(syntax_path)?;
        let print: Print = serde_json::from_str(&syntax_string)?;
        //println!("{:?}", print);
        println!("{:?}", print.states.get("s1"));
        Ok(())
    }
}

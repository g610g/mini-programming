pub mod utils {
    use std::{error::Error, fs::File, io::BufReader};
    //clear whitespaces of the string parameter and returns a new string
    pub fn clear_whitespaces(s: &str) -> String {
        s.chars().filter(|c| !c.is_whitespace()).collect()
    }
    //reads file and returns the BufReader instance for reading each line in the text
    pub fn read_file(filename: &str) -> Result<BufReader<File>, Box<dyn Error>> {
        let file = File::open(filename)?;
        let reader = BufReader::new(file);
        Ok(reader)
    }
}
pub mod core {
    use serde::{Deserialize, Serialize};
    use std::{collections::HashMap, error::Error, fs, io::BufRead};
    #[warn(dead_code)]
    struct Syntax {}
    #[derive(Serialize, Deserialize, Debug)]
    struct Print {
        states: HashMap<String, HashMap<char, String>>,
        accept_state: String,
    }
    pub fn start(filename: &str) -> Result<(), Box<dyn Error>> {
        let print_struct: Print;
        let mut struct_empty = true;
        let modifed_filename = "assets/".to_string() + filename;
        match init() {
            Ok(p) => {
                print_struct = p;
                struct_empty = false;
                //println!("{:?}", print_struct);
            }
            Err(e) => {
                println!("Error: {}", e);
                return Err(e);
            }
        }
        if struct_empty {
            println!("hello world");
            return Err("THIS WILL BE HANDLED".into());
        }
        //modify by appending .txt into the filename passed
        if !fs::metadata(&modifed_filename)?.is_file() {
            return Err("file does not exist in the directory assets!".into());
        } else {
            //read the syntax of it
            let reader = super::utils::read_file(&modifed_filename)?;
            for line in reader.lines() {
                process(&print_struct, line?)
            }
            return Ok(());
        }
    }
    fn init() -> Result<Print, Box<dyn Error>> {
        let syntax_path = "assets/syntax.json";
        let syntax_string = fs::read_to_string(syntax_path)?;
        let print: Print = serde_json::from_str(&syntax_string)?;
        Ok(print)
    }
    #[warn(dead_code)]
    //processes the syntax_syntax based on the print syntax of my langauge
    fn process(print_struct: &Print, syntax_string: String) {
        let mut state: &str = "s1";
        for character in syntax_string.chars() {
            //start from the starting state and move through on each syntax
            state = print_struct
                .states
                .get(state)
                .unwrap()
                .get(&character)
                .unwrap();
        }
        if state == print_struct.accept_state {
            println!("Syntax accepted!");
        }
    }
}

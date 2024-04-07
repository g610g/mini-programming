pub mod utils {
    use std::{error::Error, fs::File, io::BufReader, usize};
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
    pub fn find_index_bound(character_pattern: char, search_string: &str) -> Vec<usize> {
        search_string
            .match_indices(character_pattern)
            .map(|(index, _)| index)
            .collect()
    }
}
pub mod core {
    use crate::utils;
    use serde::{Deserialize, Serialize};
    use std::{collections::HashMap, error::Error, fs, io::BufRead};
    #[warn(dead_code)]
    struct Syntax {}
    #[derive(Serialize, Deserialize, Debug)]
    struct Print {
        states: HashMap<String, HashMap<char, String>>,
        accept_state: String,
        syntax_characters: Vec<char>,
    }
    pub fn start(filename: &str) -> Result<(), Box<dyn Error>> {
        let print_struct: Print;
        let mut struct_empty = true;
        let modifed_filename = "assets/".to_string() + filename;
        match init() {
            Ok(p) => {
                print_struct = p;
                struct_empty = false;
            }
            Err(e) => {
                println!("Error: {}", e);
                return Err(e);
            }
        }
        if struct_empty {
            return Err("Struct is empty".into());
        }
        //modify by appending .txt into the filename passed
        if !fs::metadata(&modifed_filename)?.is_file() {
            return Err("file does not exist in the directory assets!".into());
        } else {
            //read the syntax of it
            let reader = utils::read_file(&modifed_filename)?;
            for line in reader.lines() {
                let line = line?;
                process(&print_struct, utils::clear_whitespaces(&line))?;
            }
            return Ok(());
        }
    }
    //prepares program to deserialize table from the json file
    fn init() -> Result<Print, Box<dyn Error>> {
        let syntax_path = "assets/syntax.json";
        let syntax_string = fs::read_to_string(syntax_path)?;
        let print: Print = serde_json::from_str(&syntax_string)?;
        Ok(print)
    }
    //processes the syntax_syntax based on the print syntax of my langauge
    fn process(print_struct: &Print, syntax_string: String) -> Result<(), Box<dyn Error>> {
        let mut state: &str = "s1";
        for character in syntax_string.chars() {
            //start from the starting state and move through on each syntax
            if !print_struct.syntax_characters.contains(&character) {
                state = print_struct.states.get(state).unwrap().get(&'@').unwrap();
            } else {
                state = print_struct
                    .states
                    .get(state)
                    .unwrap()
                    .get(&character)
                    .unwrap();
            }
        }
        if state != print_struct.accept_state {
            return Err("SYNTAX MAY BE WRONG!".into());
        } else {
            //finds the index of the string to be matched
            let indices = utils::find_index_bound('\"', &syntax_string);
            //filters using iterator to only retain the character within the qoute mark exclusive ""
            let string_print: String = syntax_string
                .char_indices()
                .filter_map(|(index, character)| {
                    if index > indices[0] && index < indices[1] {
                        Some(character)
                    } else {
                        None
                    }
                })
                .collect();
            println!("{}", string_print);
            Ok(())
        }
    }
}

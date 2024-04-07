pub mod utils {
    use std::{error::Error, fs::File, io::BufReader, usize};
    //clear whitespaces of the string parameter and returns a new string
    pub fn clear_whitespaces_print(s: String) -> Result<String, &'static str> {
        //clears white spaces except string within the qoute
        let index_bounds = find_index_bound('\"', &s);
        if index_bounds.len() < 2 {
            Err("SYNTAX ERROR!")
        } else {
            Ok(s.char_indices()
                .filter_map(|(index, c)| {
                    if c.is_whitespace() && (index < index_bounds[0] || index > index_bounds[1]) {
                        None
                    } else {
                        Some(c)
                    }
                })
                .collect())
        }
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
    #[derive(Serialize, Deserialize, Debug)]
    struct Syntax {
        print: Print,
        operation: Operation,
    }
    #[derive(Serialize, Deserialize, Debug)]
    struct Print {
        states: HashMap<String, HashMap<char, String>>,
        accept_state: String,
        syntax_characters: Vec<char>,
    }
    #[derive(Serialize, Deserialize, Debug)]
    struct Operation {
        states: HashMap<String, HashMap<char, String>>,
        accept_state: String,
        syntax_characters: Vec<char>,
    }
    pub fn start(filename: &str) -> Result<(), Box<dyn Error>> {
        let syntax: Syntax;
        let mut struct_empty = true;
        let mut modifed_filename = "assets/".to_string() + filename;
        match init() {
            Ok(s) => {
                syntax = s;
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
        let file_meta: fs::Metadata;
        match fs::metadata(&modifed_filename) {
            Ok(meta) => file_meta = meta,
            Err(_e) => {
                modifed_filename = modifed_filename + ".txt";
                if let Ok(meta) = fs::metadata(&modifed_filename) {
                    file_meta = meta;
                } else {
                    return Err("cannot find directory or file".into());
                }
            }
        }
        if !file_meta.is_file() {
            return Err("file name is not a file".into());
        } else {
            //read the syntax of it
            let reader = utils::read_file(&modifed_filename)?;
            for line in reader.lines() {
                let line = utils::clear_whitespaces_print(line?)?;
                process_print(&syntax, line)?;
            }
            return Ok(());
        }
    }
    //prepares program to deserialize table from the json file
    fn init() -> Result<Syntax, Box<dyn Error>> {
        let syntax_path = "assets/syntax.json";
        let syntax_string = fs::read_to_string(syntax_path)?;
        let syntax: Syntax = serde_json::from_str(&syntax_string)?;
        Ok(syntax)
    }
    //processes the syntax_syntax based on the print syntax of my langauge
    fn process_print(syntax: &Syntax, syntax_string: String) -> Result<(), Box<dyn Error>> {
        let mut state: &str = "s1";
        for character in syntax_string.chars() {
            //start from the starting state and move through on each syntax
            if !syntax.print.syntax_characters.contains(&character) {
                state = syntax.print.states.get(state).unwrap().get(&'@').unwrap();
            } else {
                state = syntax
                    .print
                    .states
                    .get(state)
                    .unwrap()
                    .get(&character)
                    .unwrap();
            }
        }
        if state != syntax.print.accept_state {
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
            print!("{}", string_print);
            Ok(())
        }
    }
    fn process_operation(op: &Operation, string_operation: String) {
        let mut state: &str = "s1";
        for character in string_operation.chars() {
            if character.is_numeric() {
                state = op.states.get(state).unwrap().get(&'@').unwrap();
            } else {
                state = op.states.get(state).unwrap().get(&character).unwrap();
            }
        }
        println!("{}", string_operation);
    }
}

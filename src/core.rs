use super::core::Operator::*;
use crate::utils;
use core::f32;
use serde::{Deserialize, Serialize};
use std::{char, collections::HashMap, error::Error, fs, io::BufRead};
#[derive(Serialize, Deserialize, Debug)]
struct Syntax {
    pub print: Print,
    pub operation: Operation,
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
trait Process {
    fn process_syntax(&self, syntax_string: String) -> Result<(), Box<dyn Error>>;
    fn clear_whitespace(&self, s: String) -> Result<String, &'static str>;
}
#[derive(Debug)]
enum Operator {
    Add(String),
    Substract(String),
    Multiply(String),
    Divide(String),
    Modulo(String),
}
impl Operator {
    pub fn extract_operator(string_operation: &str) -> Option<Self> {
        let operator: String = string_operation
            .chars()
            .filter(|e| !e.is_numeric() && *e != '.')
            .collect();
        if operator == "+" {
            return Some(Add(operator));
        } else if operator == "-" {
            return Some(Substract(operator));
        } else if operator == "*" {
            return Some(Multiply(operator));
        } else if operator == "/" {
            return Some(Divide(operator));
        } else if operator == "%" {
            return Some(Modulo(operator));
        } else {
            None
        }
    }
}
impl Process for Print {
    fn process_syntax(&self, syntax_string: String) -> Result<(), Box<dyn Error>> {
        let mut state: &str = "s1";
        for character in syntax_string.chars() {
            //start from the starting state and move through on each syntax
            if !self.syntax_characters.contains(&character) {
                state = self.states.get(state).unwrap().get(&'@').unwrap();
            } else {
                state = self.states.get(state).unwrap().get(&character).unwrap();
            }
        }
        if state != self.accept_state {
            return Err("SYNTAX MAY BE WRONG!".into());
        } else {
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
    fn clear_whitespace(&self, s: String) -> Result<String, &'static str> {
        let index_bounds = utils::find_index_bound('\"', &s);
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
}
impl Process for Operation {
    fn process_syntax(&self, syntax_string: String) -> Result<(), Box<dyn Error>> {
        let mut state: &str = "s1";
        let syntax_string = self.clear_whitespace(syntax_string)?;
        for character in syntax_string.chars() {
            if character.is_numeric() || character == '.' {
                state = self.states.get(state).unwrap().get(&'@').unwrap();
            } else if !self.syntax_characters.contains(&character) {
                return Err("Invalid Operand Syntax".into());
            } else {
                state = self.states.get(state).unwrap().get(&character).unwrap();
            }
        }
        if state != self.accept_state {
            return Err("Syntax Error".into());
        }

        let numeric_string: String = syntax_string
            .chars()
            .map(|c| if !c.is_numeric() && c != '.' { ' ' } else { c })
            .collect();
        let operator: Operator;
        match Operator::extract_operator(&syntax_string) {
            Some(op) => operator = op,
            None => return Err("syntax operator error!".into()),
        }
        if utils::is_float(&numeric_string) {
            self.float_operation(operator, &numeric_string);
            return Ok(());
        }
        self.i32_operation(operator, &numeric_string)?;
        Ok(())
    }
    fn clear_whitespace(&self, s: String) -> Result<String, &'static str> {
        Ok(s.chars().filter(|c| !c.is_whitespace()).collect())
    }
}
impl Operation {
    fn i32_operation(
        &self,
        operator: Operator,
        numeric_string: &str,
    ) -> Result<(), Box<dyn Error>> {
        let splitted: Vec<_> = numeric_string
            .split(" ")
            .filter_map(|s| s.parse::<i32>().ok())
            .collect();
        if splitted.len() < 2 {
            return Err("The operand must be at the same data type".into());
        }
        let mut split_iter = splitted.into_iter();
        match operator {
            Add(_a) => {
                println!("{}", split_iter.sum::<i32>());
            }
            Multiply(_m) => {
                println!("{}", split_iter.product::<i32>());
            }
            Substract(_s) => {
                let mut difference: i32 = split_iter.next().unwrap();
                for element in split_iter {
                    difference -= element;
                }
                println!("{}", difference);
            }
            Divide(_d) => {
                let mut qoutient: i32 = split_iter.next().unwrap();
                for element in split_iter {
                    qoutient /= element;
                }
                println!("{}", qoutient);
            }
            Modulo(_mo) => {
                let mut result: i32 = split_iter.next().unwrap();
                for element in split_iter {
                    result %= element;
                }
                println!("{}", result);
            }
        }
        Ok(())
    }
    fn float_operation(&self, operator: Operator, numeric_string: &str) {
        let splitted: Vec<_> = numeric_string
            .split(" ")
            .filter_map(|s| s.parse::<f32>().ok())
            .collect();
        let mut split_iter = splitted.into_iter();
        match operator {
            Add(_a) => {
                println!("{}", split_iter.sum::<f32>());
            }
            Multiply(_m) => {
                println!("{}", split_iter.product::<f32>());
            }
            Substract(_s) => {
                let mut difference: f32 = split_iter.next().unwrap();
                for element in split_iter {
                    difference -= element;
                }
                println!("{}", difference);
            }
            Divide(_d) => {
                let mut qoutient: f32 = split_iter.next().unwrap();
                for element in split_iter {
                    qoutient /= element;
                }
                println!("{}", qoutient);
            }
            Modulo(_mo) => {
                let mut result: f32 = split_iter.next().unwrap();
                for element in split_iter {
                    result %= element;
                }
                println!("{}", result);
            }
        }
    }
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
            let line = line?;
            let first_element: char;
            match line.chars().nth(0) {
                Some(f) => first_element = f,
                None => return Err("Error".into()),
            }
            if first_element.is_numeric() {
                syntax.operation.process_syntax(line)?;
            } else {
                syntax.print.process_syntax(line)?;
            }
        }
        return Ok(());
    }
}
//deserialize json data into the Syntax struct
fn init() -> Result<Syntax, Box<dyn Error>> {
    let syntax_path = "assets/syntax.json";
    let syntax_string = fs::read_to_string(syntax_path)?;
    let syntax: Syntax = serde_json::from_str(&syntax_string)?;
    Ok(syntax)
}

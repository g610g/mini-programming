pub mod core;
pub mod utils {
    use std::{error::Error, fs::File, io::BufReader, usize};
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
    pub fn is_float(numeric_string: &str) -> bool {
        let mut is_float = true;
        numeric_string.split(" ").for_each(|s| {
            if s.contains(".") {
                is_float = true && is_float
            } else {
                is_float = false && is_float
            }
        });
        return is_float;
    }
}

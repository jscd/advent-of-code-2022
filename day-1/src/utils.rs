use std::fs;

pub fn get_file(path: &str) -> String {
    fs::read_to_string(path).unwrap()
}

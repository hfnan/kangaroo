use std::fs;

pub fn run(filepath: &String) {
    let lines = fs::read_to_string(filepath).expect(&format!("No file {filepath}"));

}
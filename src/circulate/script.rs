use std::fs;
use crate::compiler::producer;

pub fn run(filepath: &String) {
    let lines = fs::read_to_string(filepath).expect(&format!("No file {filepath}"));
    for line in lines.split('\n') {
        producer::run(line.to_owned());
    }
}
use std::fs;
use crate::compiler::producer;

pub fn run(filepath: &String) {
    let lines = fs::read_to_string(filepath).expect(&format!("No file {filepath}"));
    // TODO

    for line in lines.split('\n') {
        let mut line = line.to_owned();
        line.push(';');
        producer::run(line.to_owned());
    }
}
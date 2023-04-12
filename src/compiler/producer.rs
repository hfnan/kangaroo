use super::toucher;
use super::parser;

pub fn run(line: String) {
    let line = toucher::scratch(line).unwrap_or("panic(\"Even cannot scratch it!\")".to_owned());
    parser::parse(line);
}
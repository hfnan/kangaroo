use crate::compiler::producer;
use std::io::{self, Read, Write};

pub fn run() {
    println!("Kangaroo v0.0.1");
    println!("Welcome!");

    loop {
        let mut sentence = String::new();
        
        print!(">>> ");
        io::stdout().flush().unwrap();

        io::stdin().read_line(&mut sentence).unwrap();

        // println!("{}", sentence);
        producer::run(sentence);
    }
}

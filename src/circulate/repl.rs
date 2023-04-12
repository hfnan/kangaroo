use std::io::{self, Write};
use crate::compiler::producer;

pub fn run() {
    println!("Kangaroo v0.0.1");
    println!("Welcome!");
    
    let mut cnt = 0;
    loop {
        print!("$[{cnt}] ");
        io::stdout().flush().expect("Cannot flush output!");
        cnt += 1;

        let mut line = String::new();
        io::stdin().read_line(&mut line).expect("Cannot read from command!");        

        producer::run(line);        
    }
}
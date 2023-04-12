mod circulate;
mod compiler;
use circulate::{repl, script};
use std::{env, process};

fn main() {
    let args: Vec<String> = env::args().collect();

    match args.len() {
        1 => repl::run(),
        2 => script::run(&args[1]),
        _ => println!("Problem passing arguments: Too many arguments")
    }
}

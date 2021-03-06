extern crate rlox;
use rlox::lox;

use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() > 2 {
        println!("Usage: rlox [script]");
    }
    else if args.len() == 2 {
        lox::run_file(&args[1]);
    }
    else {
        lox::run_prompt();
    }
}

#[macro_use] extern crate maplit;

mod scanner;
mod token;

use std::fs::File;
use std::io::prelude::*;
use std::io;

use scanner::Scanner;

pub struct Lox {
    had_error: bool
}

impl Lox {
    pub fn new() -> Self {
        Lox { had_error: false }
    }

    pub fn run_file(&mut self, path: &str) {
        let mut f = File::open(path)
            .expect(&format!("Path not found: {}", path));
        
        let mut contents = String::new();
        f.read_to_string(&mut contents)
            .expect("Failed reading contents from file!");
        
        self.run(&contents);
        if self.had_error { std::process::exit(65); }
    }

    pub fn run_prompt(&mut self) {
        loop {
            let mut line = String::new();
            io::stdin().read_line(&mut line)
                .expect("Failed to read line");
            self.run(&line);
            self.had_error = false;
        }
    }

    pub fn run(&mut self, source: &str) {
        let mut scanner = Scanner::new(source, self);
        let tokens = scanner.scan_tokens();
        for token in tokens {
            println!("{}", token);
        }
    }

    pub fn error(&mut self, line: usize, message: &str) {
        self.report(line, "", message);
    }

    pub fn report(&mut self, line: usize, at: &str, message: &str) {
        println!("[line {line}] Error{at}: {message}", line=line, at=at, message=message);
        self.had_error = true;
    }
}

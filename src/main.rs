use std::{
    env, fs,
    io::{self, BufRead},
    process,
};

mod scanner;
mod token;

use crate::{scanner::Scanner, token::Token};

fn run(line: String) -> () {
    let scanner = Scanner::new(line);
    let tokens: Vec<Token> = scanner.scan_tokens();

    println!("{:?}", tokens);
}

fn error(line: usize, message: String) -> () {
    report(line, "", message)
}

fn report(line: usize, at: &str, message: String) {
    println!("[line {}] Error {}: {}", line, at, message)
}

fn run_prompt() -> () {
    let stdin = io::stdin();
    let mut input_lines = stdin.lock().lines();

    loop {
        print!("> ");

        match input_lines.next() {
            Some(Ok(line)) => {
                run(line);
            }
            Some(Err(e)) => {
                eprintln!("error while reading line: {}", e);
            }
            None => todo!(),
        }
    }
}

fn run_file(file: String) -> () {
    match fs::read_to_string(file) {
        Ok(content) => run(content),
        Err(e) => eprintln!("error while loading file: {}", e),
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();

    match args.len() {
        n if n > 2 => {
            println!("Usage: rlox [file]");
            process::exit(64);
        }
        2 => {
            run_file(args[1].clone());
        }
        _ => run_prompt(),
    }
}

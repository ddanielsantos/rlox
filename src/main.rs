use std::{env, process};

fn run_prompt() -> () {
    println!("Showing some options");
    ()
}

fn run_file(file: String) -> () {
    println!("Sucessfully running");
    ()
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

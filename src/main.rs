use std::{
    env,
    io::{self, BufRead},
    process,
};

fn run(line: String) -> () {
    println!("you passed {}", line);
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

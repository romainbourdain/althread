use std::{env::args, process::exit};

use althread_with_pest::{run_file, run_prompt};

fn main() {
    let args: Vec<String> = args().collect();

    match args.len() {
        1 => run_prompt(),
        2 => run_file(&args[1]).expect("Could not run file"),
        _ => {
            eprintln!("Usage: {} [script]", args[0]);
            exit(1);
        }
    }
}

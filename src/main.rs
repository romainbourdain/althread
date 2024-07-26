use std::{env::args, process::exit};

use althread_with_pest::run_file;

fn main() {
    let args: Vec<String> = args().collect();

    match args.len() {
        2 => run_file(&args[1]).expect("Could not run file"),
        _ => {
            eprintln!("Usage: {} <script.alt>", args[0]);
            exit(1);
        }
    }
}

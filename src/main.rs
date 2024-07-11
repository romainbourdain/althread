use std::{env::args, io, process::exit};

use althread_with_pest::{run_file, run_prompt};

fn main() {
    let mut output = io::stdout();
    let args: Vec<String> = args().collect();

    match args.len() {
        1 => run_prompt(&mut output),
        2 => run_file(&args[1], &mut output).expect("Could not run file"),
        _ => {
            eprintln!("Usage: {} [script]", args[0]);
            exit(1);
        }
    }
}

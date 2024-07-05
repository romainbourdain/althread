mod ast;
mod env;
mod error;
mod parser;

use std::{
    fs,
    io::{self, Write},
    process::exit,
};

use ast::program::Program;
use env::Environment;
use error::AlthreadError;
use parser::parse;

pub fn run_file(path: &str) -> io::Result<()> {
    let buf = fs::read_to_string(path)?;
    if let Err(_) = run(buf) {
        exit(1);
    }
    Ok(())
}

pub fn run_prompt() {
    let stdin = io::stdin();
    let mut stdout = io::stdout();

    loop {
        print!("> ");
        stdout.flush().unwrap();

        let mut line = String::new();
        if stdin.read_line(&mut line).unwrap() == 0 || line.trim().is_empty() {
            break;
        }

        if let Err(e) = run(line) {
            e.report("".to_string());
        }
    }
}

fn run(source: String) -> Result<(), AlthreadError> {
    let pairs = parse(&source).map_err(|e| {
        e.report("Syntax Error".to_string());
        e
    })?;

    let mut env = Environment::new();
    env.push_table();

    let prog = Program::build(pairs, &mut env).map_err(|e| {
        e.report("Semantic Error".to_string());
        e
    })?;

    env.pop_table();
    println!("{:#?}", prog);
    Ok(())
}

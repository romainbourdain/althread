mod ast;
mod env;
mod error;
mod nodes;
mod parser;
mod runtime;

use std::{
    fs,
    io::{self, Write},
    process::exit,
};

use env::{symbol_table::SymbolTable, Environment};
use error::AlthreadError;
use nodes::Ast;
use parser::parse;

/// Run code from file
pub fn run_file(path: &str) -> io::Result<()> {
    let buf = fs::read_to_string(path)?;
    if let Err(e) = run(&buf) {
        e.report(&buf);
        exit(1);
    }
    Ok(())
}

/// Run code with the client
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

        if let Err(e) = run(&line) {
            e.report(&line)
        }
    }
}

fn run(source: &str) -> Result<(), AlthreadError> {
    // parse code with pest
    let pairs = parse(&source).map_err(|e| e)?;

    let mut global_table = SymbolTable::new();
    let mut env = Environment::new(&mut global_table);

    // create ast
    let ast = Ast::build(pairs, &mut env)?;

    env.clear_global();

    // run ast
    ast.eval(&mut env)?;

    Ok(())
}

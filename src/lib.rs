mod ast;
mod env;
mod error;
mod parser;
mod runtime;

use std::{
    fs,
    io::{self, Write},
    process::exit,
};

use ast::Ast;
use env::{symbol_table::SymbolTable, Environment};
use error::AlthreadError;
use parser::parse;

/// Run code from file
pub fn run_file<W>(path: &str, output: &mut W) -> io::Result<()>
where
    W: Write,
{
    let buf = fs::read_to_string(path)?;
    if let Err(e) = run(&buf, output) {
        e.report(&buf);
        exit(1);
    }
    Ok(())
}

/// Run code with the client
pub fn run_prompt<W>(output: &mut W)
where
    W: Write,
{
    let stdin = io::stdin();
    let mut stdout = io::stdout();

    loop {
        print!("> ");
        stdout.flush().unwrap();

        let mut line = String::new();
        if stdin.read_line(&mut line).unwrap() == 0 || line.trim().is_empty() {
            break;
        }

        if let Err(e) = run(&line, output) {
            e.report(&line)
        }
    }
}

pub fn run<W>(source: &str, output: &mut W) -> Result<(), AlthreadError>
where
    W: Write,
{
    // parse code with pest
    let pairs = parse(&source).map_err(|e| e)?;

    let mut global_table = SymbolTable::new();
    let mut env = Environment::new(&mut global_table);

    // create ast
    let ast = Ast::build(pairs, &mut env)?;
    println!("{:#?}", ast);

    // env.clear_global();

    // run ast
    // ast.eval(&mut env, output)?;

    Ok(())
}

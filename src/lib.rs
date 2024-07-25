mod ast;
mod env;
mod error;
mod parser;

use std::{
    fs,
    io::{self, Write},
    process::exit,
};

use ast::Ast;
use env::{Environment, SymbolTable};
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

pub fn run<W>(source: &str, output: &mut W) -> Result<(), AlthreadError>
where
    W: Write,
{
    // parse code with pest
    let pairs = parse(&source).map_err(|e| e)?;

    // create ast
    let ast = Ast::build(pairs)?;
    println!("{}", ast);

    // check ast
    {
        println!("Checking AST...");
        let mut global_table = SymbolTable::new();
        let mut env = Environment::new(&mut global_table);

        ast.check(&mut env)?;
    }

    // run ast
    {
        println!("Running AST...");
        let mut global_table = SymbolTable::new();
        let mut env = Environment::new(&mut global_table);

        ast.eval(&mut env)?;
    }

    Ok(())
}

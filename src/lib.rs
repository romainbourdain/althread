pub mod args;
pub mod ast;
// pub mod debug;
pub mod env;
pub mod error;
pub mod parser;

use std::{cell::RefCell, rc::Rc};

use args::Config;
use ast::Ast;
use env::{Env, SymbolTableStack};
use error::AlthreadError;
use parser::parse;

pub fn run(source: &str, _config: &Config) -> Result<(), AlthreadError> {
    // parse code with pest
    let pairs = parse(&source).map_err(|e| e)?;
    let ast = Ast::build(pairs)?;

    println!("{}", ast);

    let symbol_table = Rc::new(RefCell::new(SymbolTableStack::new()));
    let mut env = Env::new(&symbol_table);
    let main_process = ast.process_blocks.get("main").unwrap();

    while !main_process.eval(&mut env)?.is_some() {}

    Ok(())
}

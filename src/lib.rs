pub mod args;
pub mod ast;
pub mod debug;
pub mod env;
pub mod error;
pub mod parser;

use args::Config;
use ast::Ast;
use error::AlthreadError;
use parser::parse;

pub fn run(source: &str, config: &Config) -> Result<(), AlthreadError> {
    // parse code with pest
    let pairs = parse(&source).map_err(|e| e)?;

    // create ast
    let ast = Ast::build(pairs)?;
    println!("{}", ast);

    // check ast
    // {
    //     let mut global_table = SymbolTable::new();
    //     let mut env = Environment::new(&mut global_table);

    //     ast.check(&mut env)?;
    // }

    // run ast
    // {
    //     let mut global_table = SymbolTable::new();
    //     let mut env = Environment::new(&mut global_table);

    //     ast.eval(&mut env, config)?;
    // }

    Ok(())
}

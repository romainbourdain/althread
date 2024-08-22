pub mod args;
pub mod ast;
// pub mod debug;
pub mod env;
pub mod error;
pub mod parser;

use args::Config;
use ast::Ast;
use env::Env;
use error::AlthreadError;
use parser::parse;

pub fn run(source: &str, config: &Config) -> Result<(), AlthreadError> {
    // parse code with pest
    let pairs = parse(&source).map_err(|e| e)?;
    let ast = Ast::build(pairs)?;

    println!("Running with seed {}\n", config.seed);
    println!("{}", ast);

    let mut env = Env::new();
    env.run(&ast, config)?;

    Ok(())
}

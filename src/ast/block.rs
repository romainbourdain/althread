use pest::iterators::Pairs;

use crate::{env::Environment, error::AlthreadError, parser::Rule};

use super::stmt::Stmt;

pub type Block = Vec<Stmt>;

pub fn parse_block(pairs: Pairs<Rule>, env: &mut Environment) -> Result<Block, AlthreadError> {
    env.push_table();
    let stmts = parse_shared_block(pairs, env)?;
    env.pop_table();
    Ok(stmts)
}

pub fn parse_shared_block(
    pairs: Pairs<Rule>,
    env: &mut Environment,
) -> Result<Block, AlthreadError> {
    let mut stmts = Vec::new();

    for pair in pairs {
        stmts.push(Stmt::build(pair, env)?);
    }

    Ok(stmts)
}

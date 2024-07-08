use pest::iterators::Pair;

use crate::{env::Environment, error::AlthreadError, parser::Rule};

use super::stmt::Stmt;

#[derive(Debug)]
pub struct Block {
    pub stmts: Vec<Stmt>,
    pub line: usize,
    pub column: usize,
}

impl Block {
    pub fn new(line: usize, column: usize) -> Self {
        Self {
            stmts: Vec::new(),
            line,
            column,
        }
    }

    pub fn parse(pair: Pair<Rule>, env: &mut Environment) -> Result<Self, AlthreadError> {
        let (line, column) = pair.line_col();
        let mut block = Self::new(line, column);

        for pair in pair.into_inner() {
            block.stmts.push(Stmt::build(pair, env)?);
        }

        Ok(block)
    }

    pub fn parse_and_push(pair: Pair<Rule>, env: &mut Environment) -> Result<Self, AlthreadError> {
        env.push_table();
        let block = Self::parse(pair, env)?;
        env.pop_table();
        Ok(block)
    }
}

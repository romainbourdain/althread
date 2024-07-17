use pest::iterators::Pair;

use crate::{env::Environment, error::AlthreadError, parser::Rule};

use super::block::Block;

#[derive(Debug)]
pub struct Process {
    pub identifier: String,
    pub block: Block,
    pub line: usize,
    pub column: usize,
}

impl Process {
    pub fn new(line: usize, column: usize) -> Self {
        Self {
            identifier: "".to_string(),
            block: Block::new(line, column),
            line,
            column,
        }
    }
}

impl Process {
    pub fn parse(pair: Pair<Rule>, env: &mut Environment) -> Result<Self, AlthreadError> {
        let (line, column) = pair.line_col();
        let mut process = Process::new(line, column);

        for pair in pair.into_inner() {
            match pair.as_rule() {
                Rule::IDENTIFIER => process.identifier = pair.as_str().to_string(),
                Rule::block => process.block = Block::parse(pair, env)?,
                _ => unreachable!(),
            }
        }

        Ok(process)
    }
}

pub mod block;
pub mod expr;
pub mod process;
pub mod stmt;
pub mod token;

use std::io::Write;

use block::Block;
use pest::iterators::Pairs;
use process::Process;

use crate::{env::Environment, error::AlthreadError, parser::Rule};

#[derive(Debug)]
pub struct Ast {
    pub main_block: Option<Block>,
    pub shared_block: Option<Block>,
    pub process_block_list: Vec<Process>,
    pub line: usize,
    pub column: usize,
}

impl Ast {
    pub fn new(line: usize, column: usize) -> Self {
        Self {
            main_block: None,
            shared_block: None,
            process_block_list: Vec::new(),
            line,
            column,
        }
    }
}

impl Ast {
    pub fn build(pairs: Pairs<Rule>, env: &mut Environment) -> Result<Self, AlthreadError> {
        let (line, column) = pairs.clone().next().unwrap().line_col();
        let mut program = Self::new(line, column);

        for pair in pairs {
            match pair.as_rule() {
                Rule::main_block => {
                    program.main_block = Some(Block::parse_and_push(pair, env)?);
                }
                Rule::shared_block => {
                    program.shared_block = Some(Block::parse(pair, env)?);
                }
                Rule::process_block => program.process_block_list.push(Process::parse(pair, env)?),
                Rule::EOI => break,
                rule => unreachable!("{:?}", rule),
            }
        }

        Ok(program)
    }

    pub fn eval<W>(&self, env: &mut Environment, output: &mut W) -> Result<(), AlthreadError>
    where
        W: Write,
    {
        if let Some(block) = self.shared_block.as_ref() {
            block.eval(env, output)?;
        }
        if let Some(block) = self.main_block.as_ref() {
            block.eval_and_push(env, output)?;
        }

        Ok(())
    }
}

pub mod assign;
pub mod block;
pub mod datatype;
pub mod decl;
pub mod expr;
pub mod if_stmt;
pub mod print_stmt;
pub mod stmt;
pub mod while_stmt;

use pest::iterators::Pairs;

use crate::{
    env::Environment,
    error::AlthreadError,
    nodes::{block::Block, Ast},
    parser::Rule,
};

impl Ast {
    pub fn build(pairs: Pairs<Rule>, env: &mut Environment) -> Result<Self, AlthreadError> {
        let (line, column) = pairs.clone().next().unwrap().line_col();
        let mut program = Self {
            main_block: None,
            shared_block: None,
            line,
            column,
        };

        for pair in pairs {
            match pair.as_rule() {
                Rule::main_block => {
                    program.main_block = Some(Block::parse_and_push(pair, env)?);
                }
                Rule::shared_block => {
                    program.shared_block = Some(Block::parse(pair, env)?);
                }
                Rule::EOI => break,
                rule => unreachable!("{:?}", rule),
            }
        }

        Ok(program)
    }
}

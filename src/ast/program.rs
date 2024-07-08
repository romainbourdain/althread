use pest::iterators::Pairs;

use crate::{env::Environment, error::AlthreadError, parser::Rule};

use super::block::Block;

#[derive(Debug)]
pub struct Program {
    pub main_block: Option<Block>,
    pub shared_block: Option<Block>,
    pub always_block: Option<Block>,
}

impl Program {
    pub fn build(pairs: Pairs<Rule>, env: &mut Environment) -> Result<Self, AlthreadError> {
        let mut main_block = None;
        let mut shared_block = None;
        let always_block = None;

        for pair in pairs {
            match pair.as_rule() {
                Rule::main_block => {
                    main_block = Some(Block::parse_and_push(pair, env)?);
                }
                Rule::shared_block => {
                    shared_block = Some(Block::parse(pair, env)?);
                }
                Rule::always_block => {
                    // TODO : implement always block
                    unimplemented!();
                }
                Rule::EOI => break,
                rule => unreachable!("{:?}", rule),
            }
        }

        Ok(Self {
            main_block,
            shared_block,
            always_block,
        })
    }
}

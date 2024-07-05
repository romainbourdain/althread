use pest::iterators::Pairs;

use crate::{env::Environment, error::AlthreadError, parser::Rule};

use super::block::{parse_block, parse_shared_block, Block};

#[derive(Debug)]
pub struct Program {
    pub main_block: Block,
    pub shared_block: Block,
    pub always_block: Block,
}

impl Program {
    pub fn build(pairs: Pairs<Rule>, env: &mut Environment) -> Result<Self, AlthreadError> {
        let mut main_block = Vec::new();
        let mut shared_block = Vec::new();
        let always_block = Vec::new();

        for pair in pairs {
            match pair.as_rule() {
                Rule::main_block => {
                    main_block = parse_block(pair.into_inner(), env)?;
                }
                Rule::shared_block => {
                    shared_block = parse_shared_block(pair.into_inner(), env)?;
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

use pest::iterators::Pairs;

use crate::{env::Environment, error::AlthreadError, parser::Rule};

use super::{
    block::{parse_block, parse_shared_block},
    stmt::Stmt,
};

#[derive(Debug)]
pub struct Program {
    pub main_block: Option<Vec<Stmt>>,
    pub shared_block: Option<Vec<Stmt>>,
    pub always_block: Option<Vec<Stmt>>,
}

impl Program {
    pub fn build(pairs: Pairs<Rule>, env: &mut Environment) -> Result<Self, AlthreadError> {
        let mut main_block = None;
        let mut shared_block = None;
        let always_block = None;

        for pair in pairs {
            match pair.as_rule() {
                Rule::main_block => {
                    main_block = Some(parse_block(pair.into_inner(), env)?);
                }
                Rule::shared_block => {
                    shared_block = Some(parse_shared_block(pair.into_inner(), env)?);
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

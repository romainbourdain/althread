use pest::iterators::Pairs;

use crate::{error::AlthreadError, parser::Rule};

use super::stmt::Stmt;

#[derive(Debug)]
pub struct Program {
    pub main_block: Option<Vec<Stmt>>,
    pub shared_block: Option<Vec<Stmt>>,
    pub always_block: Option<Vec<Stmt>>,
}

impl Program {
    pub fn build(pairs: Pairs<Rule>) -> Result<Self, AlthreadError> {
        let mut main_block = None;
        let shared_block = None;
        let always_block = None;

        for pair in pairs {
            match pair.as_rule() {
                Rule::main_block => {
                    // TODO : create main block env
                    main_block = Some(Self::parse_block(pair.into_inner())?);
                }
                Rule::shared_block => {
                    // TODO : implement shared block
                    unimplemented!();
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

    pub fn parse_block(pairs: Pairs<Rule>) -> Result<Vec<Stmt>, AlthreadError> {
        let mut stmts = Vec::new();

        for pair in pairs {
            stmts.push(Stmt::build(pair)?);
        }

        Ok(stmts)
    }
}

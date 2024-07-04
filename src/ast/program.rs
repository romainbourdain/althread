use pest::iterators::Pairs;

use crate::{env::Environment, error::AlthreadError, parser::Rule};

use super::stmt::Stmt;

#[derive(Debug)]
pub struct Program {
    pub main_block: Option<Vec<Stmt>>,
    pub shared_block: Option<Vec<Stmt>>,
    pub always_block: Option<Vec<Stmt>>,
}

impl Program {
    pub fn build(pairs: Pairs<Rule>, env: &mut Environment) -> Result<Self, AlthreadError> {
        let mut main_block = None;
        let shared_block = None;
        let always_block = None;

        for pair in pairs {
            match pair.as_rule() {
                Rule::main_block => {
                    env.push_table();
                    main_block = Some(Self::parse_block(pair.into_inner(), env)?);
                    env.pop_table();
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

    pub fn parse_block(
        pairs: Pairs<Rule>,
        env: &mut Environment,
    ) -> Result<Vec<Stmt>, AlthreadError> {
        let mut stmts = Vec::new();

        for pair in pairs {
            stmts.push(Stmt::build(pair, env)?);
        }

        Ok(stmts)
    }
}

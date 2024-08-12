use std::fmt;

use pest::iterators::Pairs;

use crate::{
    ast::{
        display::{AstDisplay, Prefix},
        node::{Build, Node},
        token::identifier::Identifier,
    },
    error::AlthreadResult,
    parser::Rule,
};

#[derive(Debug)]
pub struct RunStmt {
    pub identifier: Node<Identifier>,
}

impl Build for RunStmt {
    fn build(mut pairs: Pairs<Rule>) -> AlthreadResult<Self> {
        let identifier = Node::build_token(pairs.next().unwrap())?;

        Ok(Self { identifier })
    }
}

impl AstDisplay for RunStmt {
    fn ast_fmt(&self, f: &mut fmt::Formatter, prefix: &Prefix) -> fmt::Result {
        writeln!(f, "{prefix}run: {}", self.identifier)?;

        Ok(())
    }
}

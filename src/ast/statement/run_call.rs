use std::fmt;

use pest::iterators::Pairs;

use crate::{
    ast::{
        display::{AstDisplay, Prefix},
        node::{AstNode, Node},
        token::identifier::Identifier,
    },
    error::AlthreadResult,
    parser::Rule,
};

#[derive(Debug)]
pub struct RunCall {
    pub identifier: Node<Identifier>,
}

impl AstNode for RunCall {
    fn build(mut pairs: Pairs<Rule>) -> AlthreadResult<Self> {
        let identifier = Node::build(pairs.next().unwrap())?;

        Ok(Self { identifier })
    }
}

impl AstDisplay for RunCall {
    fn ast_fmt(&self, f: &mut fmt::Formatter, prefix: &Prefix) -> fmt::Result {
        writeln!(f, "{prefix}run: {}", self.identifier)?;

        Ok(())
    }
}

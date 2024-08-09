use std::fmt;

use pest::iterators::Pair;

use crate::{
    ast::node::{Build, Node},
    error::AlthreadResult,
    parser::Rule,
};

use super::expr::Expr;

#[derive(Debug)]
pub struct Print {
    pub value: Node<Expr>,
}

impl Build for Print {
    fn build(pair: Pair<Rule>) -> AlthreadResult<Self> {
        let mut pairs = pair.into_inner();

        let value = Node::build(pairs.next().unwrap())?;

        Ok(Self { value })
    }
}

impl fmt::Display for Print {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "print {}", self.value)
    }
}

use pest::iterators::Pair;

use crate::{
    ast::node::{Build, Node},
    error::AlthreadResult,
    parser::Rule,
};

use super::expr::primary_expr::Identifier;

#[derive(Debug)]
pub struct RunStmt {
    pub identifier: Node<Identifier>,
}

impl Build for RunStmt {
    fn build(pair: Pair<Rule>) -> AlthreadResult<Self> {
        let mut pairs = pair.into_inner();

        let identifier = Node::build(pairs.next().unwrap())?;

        Ok(Self { identifier })
    }
}

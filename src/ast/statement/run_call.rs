use std::fmt;

use pest::iterators::Pairs;

use crate::{
    ast::{
        display::{AstDisplay, Prefix},
        node::{Node, NodeBuilder, NodeExecutor},
        token::{identifier::Identifier, literal::Literal},
    },
    env::Env,
    error::AlthreadResult,
    parser::Rule,
};

#[derive(Debug)]
pub struct RunCall {
    pub identifier: Node<Identifier>,
}

impl NodeBuilder for RunCall {
    fn build(mut pairs: Pairs<Rule>) -> AlthreadResult<Self> {
        let identifier = Node::build(pairs.next().unwrap())?;

        Ok(Self { identifier })
    }
}

impl NodeExecutor for RunCall {
    fn eval(&self, _env: &mut Env) -> AlthreadResult<Option<Literal>> {
        println!("run");

        Ok(Some(Literal::Null))
    }
}

impl AstDisplay for RunCall {
    fn ast_fmt(&self, f: &mut fmt::Formatter, prefix: &Prefix) -> fmt::Result {
        writeln!(f, "{prefix}run: {}", self.identifier)?;

        Ok(())
    }
}

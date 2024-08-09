use pest::iterators::Pair;

use crate::{
    ast::{
        node::{Build, Node},
        stmt::expr::primary_expr::Identifier,
    },
    error::AlthreadResult,
    no_rule,
    parser::Rule,
};

#[derive(Debug)]
pub struct UnaryAssign {
    pub identifier: Node<Identifier>,
    pub operator: Node<UnaryAssignOp>,
}

impl Build for UnaryAssign {
    fn build(pair: Pair<Rule>) -> AlthreadResult<Self> {
        let mut pairs = pair.into_inner();

        let identifier = Node::build(pairs.next().unwrap())?;
        let operator = Node::build(pairs.next().unwrap())?;

        Ok(Self {
            identifier,
            operator,
        })
    }
}

#[derive(Debug)]
pub enum UnaryAssignOp {
    Increment,
    Decrement,
}

impl Build for UnaryAssignOp {
    fn build(pair: Pair<Rule>) -> AlthreadResult<Self> {
        match pair.as_str() {
            "++" => Ok(Self::Increment),
            "--" => Ok(Self::Decrement),
            _ => Err(no_rule!(pair)),
        }
    }
}

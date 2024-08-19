pub mod binary_assignment;
pub mod unary_assignment;

use std::fmt::{self};

use binary_assignment::BinaryAssignment;
use pest::iterators::Pairs;
use unary_assignment::UnaryAssignment;

use crate::{
    ast::{
        display::{AstDisplay, Prefix},
        node::{Node, NodeBuilder, NodeExecutor},
        token::literal::Literal,
    },
    env::process_env::ProcessEnv,
    error::AlthreadResult,
    no_rule,
    parser::Rule,
};

#[derive(Debug)]
pub enum Assignment {
    Unary(Node<UnaryAssignment>),
    Binary(Node<BinaryAssignment>),
}

impl NodeBuilder for Assignment {
    fn build(mut pairs: Pairs<Rule>) -> AlthreadResult<Self> {
        let pair = pairs.next().unwrap();

        match pair.as_rule() {
            Rule::unary_assignment => Ok(Self::Unary(Node::build(pair)?)),
            Rule::binary_assignment => Ok(Self::Binary(Node::build(pair)?)),
            _ => Err(no_rule!(pair)),
        }
    }
}

impl NodeExecutor for Assignment {
    fn eval(&self, env: &mut ProcessEnv) -> AlthreadResult<Option<Literal>> {
        match self {
            Self::Unary(node) => node.eval(env),
            Self::Binary(node) => node.eval(env),
        }
    }
}

impl AstDisplay for Assignment {
    fn ast_fmt(&self, f: &mut fmt::Formatter, prefix: &Prefix) -> fmt::Result {
        match self {
            Self::Unary(node) => node.ast_fmt(f, prefix),
            Self::Binary(node) => node.ast_fmt(f, prefix),
        }
    }
}

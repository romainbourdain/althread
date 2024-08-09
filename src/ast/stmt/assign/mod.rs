pub mod binary_assign;
pub mod unary_assign;

use std::fmt::{self};

use binary_assign::BinaryAssign;
use pest::iterators::Pair;
use unary_assign::UnaryAssign;

use crate::{
    ast::{
        display::{AstDisplay, Prefix},
        node::{Build, Node},
    },
    error::AlthreadResult,
    no_rule,
    parser::Rule,
};

#[derive(Debug)]
pub enum Assign {
    Unary(Node<UnaryAssign>),
    Binary(Node<BinaryAssign>),
}

impl Build for Assign {
    fn build(pair: Pair<Rule>) -> AlthreadResult<Self> {
        let pair = pair.into_inner().next().unwrap();

        match pair.as_rule() {
            Rule::assign_unary => Ok(Self::Unary(Node::build(pair)?)),
            Rule::assign_binary => Ok(Self::Binary(Node::build(pair)?)),
            _ => Err(no_rule!(pair)),
        }
    }
}

impl AstDisplay for Assign {
    fn ast_fmt(&self, f: &mut fmt::Formatter, prefix: &Prefix) -> fmt::Result {
        match self {
            Self::Unary(node) => node.ast_fmt(f, prefix),
            Self::Binary(node) => node.ast_fmt(f, prefix),
        }
    }
}

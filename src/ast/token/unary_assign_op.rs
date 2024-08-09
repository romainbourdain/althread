use std::fmt;

use pest::iterators::Pair;

use crate::{ast::node::Build, error::AlthreadResult, no_rule, parser::Rule};

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

impl fmt::Display for UnaryAssignOp {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Increment => write!(f, "++"),
            Self::Decrement => write!(f, "--"),
        }
    }
}

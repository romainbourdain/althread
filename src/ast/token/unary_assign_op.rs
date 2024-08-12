use std::fmt;

use pest::iterators::Pairs;

use crate::{ast::node::Build, error::AlthreadResult, no_rule, parser::Rule};

#[derive(Debug)]
pub enum UnaryAssignOp {
    Increment,
    Decrement,
}

impl Build for UnaryAssignOp {
    fn build(mut pairs: Pairs<Rule>) -> AlthreadResult<Self> {
        let pair = pairs.next().unwrap();
        match pair.as_rule() {
            Rule::inc_op => Ok(Self::Increment),
            Rule::dec_op => Ok(Self::Decrement),
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

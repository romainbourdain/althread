use std::fmt;

use pest::iterators::Pairs;

use crate::{ast::node::NodeBuilder, error::AlthreadResult, no_rule, parser::Rule};

#[derive(Debug)]
pub enum UnaryAssignmentOperator {
    Increment,
    Decrement,
}

impl NodeBuilder for UnaryAssignmentOperator {
    fn build(mut pairs: Pairs<Rule>) -> AlthreadResult<Self> {
        let pair = pairs.next().unwrap();
        match pair.as_rule() {
            Rule::INC_OP => Ok(Self::Increment),
            Rule::DEC_OP => Ok(Self::Decrement),
            _ => Err(no_rule!(pair)),
        }
    }
}

impl fmt::Display for UnaryAssignmentOperator {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Increment => write!(f, "++"),
            Self::Decrement => write!(f, "--"),
        }
    }
}

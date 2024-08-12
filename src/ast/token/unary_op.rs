use std::fmt;

use pest::iterators::Pairs;

use crate::{ast::node::Build, error::AlthreadResult, no_rule, parser::Rule};

#[derive(Debug)]
pub enum UnaryOp {
    Pos,
    Neg,
    Not,
}

impl Build for UnaryOp {
    fn build(mut pairs: Pairs<Rule>) -> AlthreadResult<Self> {
        let pair = pairs.next().unwrap();
        match pair.as_rule() {
            Rule::POS_OP => Ok(Self::Pos),
            Rule::NEG_OP => Ok(Self::Neg),
            Rule::NOT_OP => Ok(Self::Not),
            _ => Err(no_rule!(pair)),
        }
    }
}

impl fmt::Display for UnaryOp {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let op = match self {
            UnaryOp::Pos => "+",
            UnaryOp::Neg => "-",
            UnaryOp::Not => "!",
        };

        write!(f, "{}", op)
    }
}

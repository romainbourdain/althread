use std::fmt;

use pest::iterators::Pair;

use crate::{ast::node::Build, error::AlthreadResult, no_rule, parser::Rule};

#[derive(Debug)]
pub enum UnaryOp {
    Pos,
    Neg,
    Not,
}

impl Build for UnaryOp {
    fn build(pair: Pair<Rule>) -> AlthreadResult<Self> {
        match pair.as_str() {
            "+" => Ok(Self::Pos),
            "-" => Ok(Self::Neg),
            "!" => Ok(Self::Not),
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

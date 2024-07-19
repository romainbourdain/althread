use std::fmt;

use pest::iterators::Pair;

use crate::{error::AlthreadError, parser::Rule};

#[derive(Debug, Clone)]
pub enum UnOp {
    Not,
    Neg,
}

impl fmt::Display for UnOp {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use UnOp::*;
        let op = match self {
            Not => "!",
            Neg => "-",
        };
        write!(f, "{}", op)
    }
}

impl UnOp {
    pub fn from_pair(pair: Pair<Rule>) -> Result<Self, AlthreadError> {
        Ok(match pair.as_rule() {
            Rule::not => UnOp::Not,
            Rule::neg => UnOp::Neg,
            rule => unreachable!("{:?}", rule),
        })
    }
}

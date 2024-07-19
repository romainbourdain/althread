use std::fmt;

use pest::iterators::Pair;

use crate::{error::AlthreadError, parser::Rule};

#[derive(Debug, PartialEq)]
pub enum UnaryAssignOp {
    Increment,
    Decrement,
}

impl fmt::Display for UnaryAssignOp {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use UnaryAssignOp::*;
        let op = match self {
            Increment => "++",
            Decrement => "--",
        };
        write!(f, "{}", op)
    }
}

impl UnaryAssignOp {
    pub fn from_pair(pair: Pair<Rule>) -> Result<Self, AlthreadError> {
        Ok(match pair.as_str() {
            "++" => UnaryAssignOp::Increment,
            "--" => UnaryAssignOp::Decrement,
            _ => unimplemented!(),
        })
    }
}

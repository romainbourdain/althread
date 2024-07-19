use std::fmt;

use pest::iterators::Pair;

use crate::{error::AlthreadError, parser::Rule};

#[derive(Debug, PartialEq)]
pub enum AssignUnaryOp {
    Increment,
    Decrement,
}

impl fmt::Display for AssignUnaryOp {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use AssignUnaryOp::*;
        let op = match self {
            Increment => "++",
            Decrement => "--",
        };
        write!(f, "{}", op)
    }
}

impl AssignUnaryOp {
    pub fn from_pair(pair: Pair<Rule>) -> Result<Self, AlthreadError> {
        Ok(match pair.as_str() {
            "++" => AssignUnaryOp::Increment,
            "--" => AssignUnaryOp::Decrement,
            _ => unimplemented!(),
        })
    }
}

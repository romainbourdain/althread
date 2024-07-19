use std::fmt;

use pest::iterators::Pair;

use crate::{error::AlthreadError, parser::Rule};

#[derive(Debug, PartialEq)]
pub enum AssignBinaryOp {
    Assign,
    AddAssign,
    SubAssign,
    MulAssign,
    DivAssign,
    ModAssign,
}

impl fmt::Display for AssignBinaryOp {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use AssignBinaryOp::*;
        let op = match self {
            Assign => "=",
            AddAssign => "+=",
            SubAssign => "-=",
            MulAssign => "*=",
            DivAssign => "/=",
            ModAssign => "%=",
        };
        write!(f, "{}", op)
    }
}

impl AssignBinaryOp {
    pub fn from_pair(pair: Pair<Rule>) -> Result<Self, AlthreadError> {
        Ok(match pair.as_str() {
            "=" => AssignBinaryOp::Assign,
            "+=" => AssignBinaryOp::AddAssign,
            "-=" => AssignBinaryOp::SubAssign,
            "*=" => AssignBinaryOp::MulAssign,
            "/=" => AssignBinaryOp::DivAssign,
            "%=" => AssignBinaryOp::ModAssign,
            _ => unimplemented!(),
        })
    }
}

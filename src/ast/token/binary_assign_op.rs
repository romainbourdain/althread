use std::fmt;

use pest::iterators::Pair;

use crate::{error::AlthreadError, parser::Rule};

#[derive(Debug, PartialEq)]
pub enum BinaryAssignOp {
    Assign,
    AddAssign,
    SubAssign,
    MulAssign,
    DivAssign,
    ModAssign,
}

impl fmt::Display for BinaryAssignOp {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use BinaryAssignOp::*;
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

impl BinaryAssignOp {
    pub fn build(pair: Pair<Rule>) -> Result<Self, AlthreadError> {
        Ok(match pair.as_str() {
            "=" => BinaryAssignOp::Assign,
            "+=" => BinaryAssignOp::AddAssign,
            "-=" => BinaryAssignOp::SubAssign,
            "*=" => BinaryAssignOp::MulAssign,
            "/=" => BinaryAssignOp::DivAssign,
            "%=" => BinaryAssignOp::ModAssign,
            _ => unimplemented!(),
        })
    }
}

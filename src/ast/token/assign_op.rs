use std::fmt;

use pest::iterators::Pair;

use crate::{error::AlthreadError, parser::Rule};

#[derive(Debug, PartialEq)]
pub enum AssignOp {
    Assign,
    AddAssign,
    SubAssign,
    MulAssign,
    DivAssign,
    ModAssign,
}

impl fmt::Display for AssignOp {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use AssignOp::*;
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

impl AssignOp {
    pub fn build(pair: Pair<Rule>) -> Result<Self, AlthreadError> {
        Ok(match pair.as_str() {
            "=" => AssignOp::Assign,
            "+=" => AssignOp::AddAssign,
            "-=" => AssignOp::SubAssign,
            "*=" => AssignOp::MulAssign,
            "/=" => AssignOp::DivAssign,
            "%=" => AssignOp::ModAssign,
            "++" => AssignOp::AddAssign,
            "--" => AssignOp::SubAssign,
            _ => unimplemented!(),
        })
    }
}

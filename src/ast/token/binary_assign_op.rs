use std::fmt;

use pest::iterators::Pair;

use crate::{ast::node::Build, error::AlthreadResult, no_rule, parser::Rule};

#[derive(Debug)]
pub enum BinaryAssignOp {
    Assign,
    AddAssign,
    SubAssign,
    MulAssign,
    DivAssign,
    ModAssign,
}

impl Build for BinaryAssignOp {
    fn build(pair: Pair<Rule>) -> AlthreadResult<Self> {
        match pair.as_str() {
            "=" => Ok(Self::Assign),
            "+=" => Ok(Self::AddAssign),
            "-=" => Ok(Self::SubAssign),
            "*=" => Ok(Self::MulAssign),
            "/=" => Ok(Self::DivAssign),
            "%=" => Ok(Self::ModAssign),
            _ => Err(no_rule!(pair)),
        }
    }
}

impl fmt::Display for BinaryAssignOp {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Assign => write!(f, "="),
            Self::AddAssign => write!(f, "+="),
            Self::SubAssign => write!(f, "-="),
            Self::MulAssign => write!(f, "*="),
            Self::DivAssign => write!(f, "/="),
            Self::ModAssign => write!(f, "%="),
        }
    }
}

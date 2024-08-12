use std::fmt;

use pest::iterators::Pairs;

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
    fn build(mut pairs: Pairs<Rule>) -> AlthreadResult<Self> {
        let pair = pairs.next().unwrap();
        match pair.as_rule() {
            Rule::ASSIGN_OP => Ok(Self::Assign),
            Rule::ADD_ASSIGN_OP => Ok(Self::AddAssign),
            Rule::SUB_ASSIGN_OP => Ok(Self::SubAssign),
            Rule::MUL_ASSIGN_OP => Ok(Self::MulAssign),
            Rule::DIV_ASSIGN_OP => Ok(Self::DivAssign),
            Rule::MOD_ASSIGN_OP => Ok(Self::ModAssign),
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

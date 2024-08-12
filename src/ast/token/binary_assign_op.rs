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
            Rule::assign_op => Ok(Self::Assign),
            Rule::assign_add_op => Ok(Self::AddAssign),
            Rule::assign_sub_op => Ok(Self::SubAssign),
            Rule::assign_mul_op => Ok(Self::MulAssign),
            Rule::assign_div_op => Ok(Self::DivAssign),
            Rule::assign_mod_op => Ok(Self::ModAssign),
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

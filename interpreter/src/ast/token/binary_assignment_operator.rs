use std::fmt;

use pest::iterators::Pairs;

use crate::{ast::node::NodeBuilder, error::AlthreadResult, no_rule, parser::Rule};

#[derive(Debug)]
pub enum BinaryAssignmentOperator {
    Assign,
    AddAssign,
    SubtractAssign,
    MultiplyAssign,
    DivideAssign,
    ModuloAssign,
}

impl NodeBuilder for BinaryAssignmentOperator {
    fn build(mut pairs: Pairs<Rule>) -> AlthreadResult<Self> {
        let pair = pairs.next().unwrap();
        match pair.as_rule() {
            Rule::ASSIGN_OP => Ok(Self::Assign),
            Rule::ADD_ASSIGN_OP => Ok(Self::AddAssign),
            Rule::SUB_ASSIGN_OP => Ok(Self::SubtractAssign),
            Rule::MUL_ASSIGN_OP => Ok(Self::MultiplyAssign),
            Rule::DIV_ASSIGN_OP => Ok(Self::DivideAssign),
            Rule::MOD_ASSIGN_OP => Ok(Self::ModuloAssign),
            _ => Err(no_rule!(pair)),
        }
    }
}

impl fmt::Display for BinaryAssignmentOperator {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Assign => write!(f, "="),
            Self::AddAssign => write!(f, "+="),
            Self::SubtractAssign => write!(f, "-="),
            Self::MultiplyAssign => write!(f, "*="),
            Self::DivideAssign => write!(f, "/="),
            Self::ModuloAssign => write!(f, "%="),
        }
    }
}

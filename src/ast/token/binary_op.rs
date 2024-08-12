use std::fmt;

use pest::iterators::Pairs;

use crate::{ast::node::Build, error::AlthreadResult, no_rule, parser::Rule};

#[derive(Debug)]
pub enum BinaryOp {
    Add,
    Sub,
    Mul,
    Div,
    Mod,
    Eq,
    Ne,
    Lt,
    Le,
    Gt,
    Ge,
    And,
    Or,
}

impl Build for BinaryOp {
    fn build(mut pairs: Pairs<Rule>) -> AlthreadResult<Self> {
        let pair = pairs.next().unwrap();
        match pair.as_rule() {
            Rule::ADD_OP => Ok(Self::Add),
            Rule::SUB_OP => Ok(Self::Sub),
            Rule::MUL_OP => Ok(Self::Mul),
            Rule::DIV_OP => Ok(Self::Div),
            Rule::MOD_OP => Ok(Self::Mod),
            Rule::EQ_OP => Ok(Self::Eq),
            Rule::NE_OP => Ok(Self::Ne),
            Rule::LT_OP => Ok(Self::Lt),
            Rule::LE_OP => Ok(Self::Le),
            Rule::GT_OP => Ok(Self::Gt),
            Rule::GE_OP => Ok(Self::Ge),
            Rule::AND_OP => Ok(Self::And),
            Rule::OR_OP => Ok(Self::Or),
            _ => Err(no_rule!(pair)),
        }
    }
}

impl fmt::Display for BinaryOp {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let op = match self {
            BinaryOp::Add => "+",
            BinaryOp::Sub => "-",
            BinaryOp::Mul => "*",
            BinaryOp::Div => "/",
            BinaryOp::Mod => "%",
            BinaryOp::Eq => "==",
            BinaryOp::Ne => "!=",
            BinaryOp::Lt => "<",
            BinaryOp::Le => "<=",
            BinaryOp::Gt => ">",
            BinaryOp::Ge => ">=",
            BinaryOp::And => "&&",
            BinaryOp::Or => "||",
        };

        write!(f, "{}", op)
    }
}

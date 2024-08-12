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
            Rule::add_op => Ok(Self::Add),
            Rule::sub_op => Ok(Self::Sub),
            Rule::mul_op => Ok(Self::Mul),
            Rule::div_op => Ok(Self::Div),
            Rule::mod_op => Ok(Self::Mod),
            Rule::eq_op => Ok(Self::Eq),
            Rule::ne_op => Ok(Self::Ne),
            Rule::lt_op => Ok(Self::Lt),
            Rule::le_op => Ok(Self::Le),
            Rule::gt_op => Ok(Self::Gt),
            Rule::ge_op => Ok(Self::Ge),
            Rule::and_op => Ok(Self::And),
            Rule::or_op => Ok(Self::Or),
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

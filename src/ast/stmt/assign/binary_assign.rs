use pest::iterators::Pair;

use crate::{
    ast::{
        node::{Build, Node},
        stmt::expr::{primary_expr::Identifier, Expr},
    },
    error::AlthreadResult,
    no_rule,
    parser::Rule,
};

#[derive(Debug)]
pub struct BinaryAssign {
    pub identifier: Node<Identifier>,
    pub operator: Node<BinaryAssignOp>,
    pub value: Node<Expr>,
}

impl Build for BinaryAssign {
    fn build(pair: Pair<Rule>) -> AlthreadResult<Self> {
        let mut pairs = pair.into_inner();

        let identifier = Node::build(pairs.next().unwrap())?;
        let operator = Node::build(pairs.next().unwrap())?;
        let value = Node::build(pairs.next().unwrap())?;

        Ok(Self {
            identifier,
            operator,
            value,
        })
    }
}

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

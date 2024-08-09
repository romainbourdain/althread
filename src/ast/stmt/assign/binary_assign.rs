use std::fmt;

use pest::iterators::Pair;

use crate::{
    ast::{
        node::{Build, Node},
        stmt::expr::Expr,
        token::{binary_assign_op::BinaryAssignOp, identifier::Identifier},
    },
    error::AlthreadResult,
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

impl fmt::Display for BinaryAssign {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.identifier)?;
        write!(f, " {} ", self.operator)?;
        write!(f, "{}", self.value)?;

        Ok(())
    }
}

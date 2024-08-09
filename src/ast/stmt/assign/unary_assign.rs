use std::fmt;

use pest::iterators::Pair;

use crate::{
    ast::{
        node::{Build, Node},
        token::{identifier::Identifier, unary_assign_op::UnaryAssignOp},
    },
    error::AlthreadResult,
    parser::Rule,
};

#[derive(Debug)]
pub struct UnaryAssign {
    pub identifier: Node<Identifier>,
    pub operator: Node<UnaryAssignOp>,
}

impl Build for UnaryAssign {
    fn build(pair: Pair<Rule>) -> AlthreadResult<Self> {
        let mut pairs = pair.into_inner();

        let identifier = Node::build(pairs.next().unwrap())?;
        let operator = Node::build(pairs.next().unwrap())?;

        Ok(Self {
            identifier,
            operator,
        })
    }
}

impl fmt::Display for UnaryAssign {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.identifier)?;
        write!(f, " {} ", self.operator)?;

        Ok(())
    }
}

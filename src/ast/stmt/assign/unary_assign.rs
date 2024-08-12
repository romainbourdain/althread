use std::fmt::{self};

use pest::iterators::Pairs;

use crate::{
    ast::{
        display::{AstDisplay, Prefix},
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
    fn build(mut pairs: Pairs<Rule>) -> AlthreadResult<Self> {
        let identifier = Node::build_token(pairs.next().unwrap())?;
        let operator = Node::build(pairs.next().unwrap())?;

        Ok(Self {
            identifier,
            operator,
        })
    }
}

impl AstDisplay for UnaryAssign {
    fn ast_fmt(&self, f: &mut fmt::Formatter, prefix: &Prefix) -> fmt::Result {
        writeln!(f, "{}unary_assign", prefix)?;

        let prefix = &prefix.add_branch();
        writeln!(f, "{}ident: {}", prefix, self.identifier)?;

        let prefix = &prefix.switch();
        writeln!(f, "{}op: {}", prefix, self.operator)?;

        Ok(())
    }
}

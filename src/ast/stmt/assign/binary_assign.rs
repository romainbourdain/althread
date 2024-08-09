use std::fmt::{self};

use pest::iterators::Pair;

use crate::{
    ast::{
        display::{AstDisplay, Prefix},
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

impl AstDisplay for BinaryAssign {
    fn ast_fmt(&self, f: &mut fmt::Formatter, prefix: &Prefix) -> fmt::Result {
        writeln!(f, "{}binary_assign", prefix)?;

        let prefix = prefix.add_branch();
        writeln!(f, "{}ident: {}", prefix, self.identifier)?;
        writeln!(f, "{}op: {}", prefix, self.operator)?;

        let prefix = &prefix.switch();
        writeln!(f, "{}value: {}", prefix, self.operator)?;
        self.value.ast_fmt(f, prefix)?;
        Ok(())
    }
}

use std::fmt::{self};

use pest::iterators::Pair;

use crate::{
    ast::{
        display::AstDisplay,
        node::{Build, Node},
        stmt::expr::Expr,
        token::{binary_assign_op::BinaryAssignOp, identifier::Identifier},
    },
    error::AlthreadResult,
    parser::Rule,
    write_indent,
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
    fn ast_fmt(&self, f: &mut fmt::Formatter, indent_level: usize) -> fmt::Result {
        write_indent!(f, indent_level, "binary_assign")?;
        write_indent!(f, indent_level + 1, "ident: {}", self.identifier)?;
        write_indent!(f, indent_level + 1, "op: {}", self.operator)?;
        self.value.ast_fmt(f, indent_level + 1)?;
        Ok(())
    }
}

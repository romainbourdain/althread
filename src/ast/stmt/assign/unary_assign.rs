use std::fmt::{self};

use pest::iterators::Pair;

use crate::{
    ast::{
        display::AstDisplay,
        node::{Build, Node},
        token::{identifier::Identifier, unary_assign_op::UnaryAssignOp},
    },
    error::AlthreadResult,
    parser::Rule,
    write_indent,
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

impl AstDisplay for UnaryAssign {
    fn ast_fmt(&self, f: &mut fmt::Formatter, indent_level: usize) -> fmt::Result {
        write_indent!(f, indent_level, "unary_assign")?;
        write_indent!(f, indent_level + 1, "ident: {}", self.identifier)?;
        write_indent!(f, indent_level + 1, "op: {}", self.operator)?;
        Ok(())
    }
}

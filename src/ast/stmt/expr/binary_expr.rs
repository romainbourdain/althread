use std::fmt;

use pest::iterators::Pair;

use crate::{
    ast::{
        display::{AstDisplay, Prefix},
        node::{Build, Node},
        token::binary_op::BinaryOp,
    },
    error::AlthreadResult,
    parser::Rule,
};

use super::Expr;

#[derive(Debug)]
pub struct BinaryExpr {
    pub left: Box<Node<Expr>>,
    pub operator: Node<BinaryOp>,
    pub right: Box<Node<Expr>>,
}

impl Build for BinaryExpr {
    fn build(pair: Pair<Rule>) -> AlthreadResult<Self> {
        let mut pairs = pair.into_inner();

        let left = Node::build(pairs.next().unwrap())?;
        let operator = Node::build(pairs.next().unwrap())?;
        let right = Node::build(pairs.next().unwrap())?;

        Ok(Self {
            left: Box::new(left),
            operator,
            right: Box::new(right),
        })
    }
}

impl AstDisplay for BinaryExpr {
    fn ast_fmt(&self, f: &mut fmt::Formatter, prefix: &Prefix) -> fmt::Result {
        writeln!(f, "{prefix}binary_expr")?;

        let prefix = &prefix.add_branch();
        writeln!(f, "{}left", prefix)?;
        self.left.ast_fmt(f, &prefix.add_leaf())?;

        writeln!(f, "{}op: {}", prefix, self.operator)?;

        let prefix = &prefix.switch();
        writeln!(f, "{}right", prefix)?;
        self.right.ast_fmt(f, &prefix.add_leaf())?;

        Ok(())
    }
}

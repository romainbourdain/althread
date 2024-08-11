use std::fmt;

use pest::iterators::Pair;

use crate::{
    ast::{
        display::{AstDisplay, Prefix},
        node::{Build, Node},
        token::unary_op::UnaryOp,
    },
    parser::Rule,
};

use super::Expr;

#[derive(Debug)]
pub struct UnaryExpr {
    pub operator: Node<UnaryOp>,
    pub operand: Box<Node<Expr>>,
}

impl Build for UnaryExpr {
    fn build(pair: Pair<Rule>) -> crate::error::AlthreadResult<Self> {
        let mut pairs = pair.into_inner();

        let operator = Node::build(pairs.next().unwrap())?;
        let operand = Node::build(pairs.next().unwrap())?;

        Ok(Self {
            operator,
            operand: Box::new(operand),
        })
    }
}

impl AstDisplay for UnaryExpr {
    fn ast_fmt(&self, f: &mut fmt::Formatter, prefix: &Prefix) -> fmt::Result {
        writeln!(f, "{}unary_expr", prefix)?;
        let prefix = &prefix.add_branch();
        writeln!(f, "{}op: {}", prefix, self.operator)?;

        let prefix = &prefix.switch();
        writeln!(f, "{}expr", prefix)?;
        self.operand.ast_fmt(f, &prefix.add_leaf())?;

        Ok(())
    }
}

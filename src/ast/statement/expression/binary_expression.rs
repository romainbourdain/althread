use std::fmt;

use pest::iterators::Pair;

use crate::{
    ast::{
        display::{AstDisplay, Prefix},
        node::Node,
        token::binary_operator::BinaryOperator,
    },
    error::AlthreadResult,
    parser::Rule,
};

use super::Expression;

#[derive(Debug)]
pub struct BinaryExpression {
    pub left: Box<Node<Expression>>,
    pub operator: Node<BinaryOperator>,
    pub right: Box<Node<Expression>>,
}

impl BinaryExpression {
    pub fn build(
        left: Node<Expression>,
        operator: Pair<Rule>,
        right: Node<Expression>,
    ) -> AlthreadResult<Node<Self>> {
        Ok(Node {
            line: operator.line_col().0,
            column: operator.line_col().1,
            value: Self {
                left: Box::new(left),
                operator: Node::build(operator)?,
                right: Box::new(right),
            },
        })
    }
}

impl AstDisplay for BinaryExpression {
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
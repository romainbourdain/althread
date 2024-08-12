use std::fmt;

use pest::iterators::Pair;

use crate::{
    ast::{
        display::{AstDisplay, Prefix},
        node::Node,
        token::unary_operator::UnaryOperator,
    },
    error::AlthreadResult,
    parser::Rule,
};

use super::Expression;

#[derive(Debug)]
pub struct UnaryExpression {
    pub operator: Node<UnaryOperator>,
    pub operand: Box<Node<Expression>>,
}

impl UnaryExpression {
    pub fn build(operator: Pair<Rule>, operand: Node<Expression>) -> AlthreadResult<Node<Self>> {
        Ok(Node {
            line: operator.line_col().0,
            column: operator.line_col().1,
            value: Self {
                operator: Node::build(operator)?,
                operand: Box::new(operand),
            },
        })
    }
}

impl AstDisplay for UnaryExpression {
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

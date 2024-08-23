use std::fmt;

use pest::iterators::Pair;

use crate::{
    ast::{
        display::{AstDisplay, Prefix},
        node::{Node, NodeExecutor},
        token::binary_operator::BinaryOperator,
    },
    env::{node_result::NodeResult, process_env::ProcessEnv},
    error::{AlthreadError, AlthreadResult, ErrorType},
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

impl NodeExecutor for BinaryExpression {
    fn eval(&self, env: &mut ProcessEnv) -> AlthreadResult<Option<NodeResult>> {
        let left = self.left.eval(env)?.unwrap().get_literal();
        let right = self.right.eval(env)?.unwrap().get_literal();

        match self.operator.value {
            BinaryOperator::Add => left.add(&right),
            BinaryOperator::Subtract => left.subtract(&right),
            BinaryOperator::Multiply => left.multiply(&right),
            BinaryOperator::Divide => left.divide(&right),
            BinaryOperator::Modulo => left.modulo(&right),
            BinaryOperator::Equals => left.equals(&right),
            BinaryOperator::NotEquals => left.not_equals(&right),
            BinaryOperator::LessThan => left.less_than(&right),
            BinaryOperator::LessThanOrEqual => left.less_than_or_equal(&right),
            BinaryOperator::GreaterThan => left.greater_than(&right),
            BinaryOperator::GreaterThanOrEqual => left.greater_than_or_equal(&right),
            BinaryOperator::And => left.and(&right),
            BinaryOperator::Or => left.or(&right),
        }
        .map(|res| Some(NodeResult::Literal(res)))
        .map_err(|e| {
            AlthreadError::new(
                ErrorType::TypeError,
                self.operator.line,
                self.operator.column,
                format!("{}", e),
            )
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

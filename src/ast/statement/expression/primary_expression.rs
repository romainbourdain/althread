use std::fmt::{self, Debug};

use pest::iterators::Pair;

use crate::{
    ast::{
        display::AstDisplay,
        node::Node,
        token::{identifier::Identifier, literal::Literal},
    },
    error::AlthreadResult,
    no_rule,
    parser::Rule,
};

use super::Expression;

#[derive(Debug)]
pub enum PrimaryExpression {
    Literal(Node<Literal>),
    Identifier(Node<Identifier>),
    Expr(Box<Node<Expression>>),
}

impl PrimaryExpression {
    pub fn build(pair: Pair<Rule>) -> AlthreadResult<Node<Self>> {
        Ok(Node {
            line: pair.line_col().0,
            column: pair.line_col().1,
            value: match pair.as_rule() {
                Rule::literal => Self::Literal(Node::build(pair)?),
                Rule::identifier => Self::Identifier(Node::build(pair)?),
                Rule::expression => Self::Expr(Box::new(Node::build(pair)?)),
                _ => return Err(no_rule!(pair)),
            },
        })
    }
}

impl AstDisplay for PrimaryExpression {
    fn ast_fmt(&self, f: &mut fmt::Formatter, prefix: &crate::ast::display::Prefix) -> fmt::Result {
        match self {
            Self::Literal(node) => node.ast_fmt(f, prefix),
            PrimaryExpression::Identifier(value) => writeln!(f, "{prefix}ident: {value}"),
            PrimaryExpression::Expr(node) => node.ast_fmt(f, prefix),
        }
    }
}

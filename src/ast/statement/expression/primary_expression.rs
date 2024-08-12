use std::{
    fmt::{self, Debug},
    str::FromStr,
};

use pest::iterators::Pair;

use crate::{
    ast::{display::AstDisplay, node::Node, token::identifier::Identifier},
    error::{AlthreadError, AlthreadResult, ErrorType},
    no_rule,
    parser::Rule,
};

use super::Expression;

#[derive(Debug)]
pub enum PrimaryExpression {
    Null(),
    Bool(bool),
    Int(i64),
    Float(f64),
    String(String),
    Identifier(Node<Identifier>),
    Expr(Box<Node<Expression>>),
}

impl PrimaryExpression {
    pub fn build(pair: Pair<Rule>) -> AlthreadResult<Node<Self>> {
        fn parse_with_error<T: FromStr>(pair: Pair<Rule>) -> AlthreadResult<T> {
            let (line, column) = pair.line_col();
            pair.as_str().parse::<T>().map_err(|_| {
                AlthreadError::new(
                    ErrorType::SyntaxError,
                    line,
                    column,
                    format!("Cannot parse {}", pair.as_str()),
                )
            })
        }

        Ok(Node {
            line: pair.line_col().0,
            column: pair.line_col().1,
            value: match pair.as_rule() {
                Rule::NULL => Self::Null(),
                Rule::BOOL => Self::Bool(parse_with_error::<bool>(pair)?),
                Rule::INT => Self::Int(parse_with_error::<i64>(pair)?),
                Rule::FLOAT => Self::Float(parse_with_error::<f64>(pair)?),
                Rule::STR => Self::String(pair.as_str().to_string()),
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
            PrimaryExpression::Null() => writeln!(f, "{prefix}null"),
            PrimaryExpression::Bool(value) => writeln!(f, "{prefix}bool: {value}"),
            PrimaryExpression::Int(value) => writeln!(f, "{prefix}int: {value}"),
            PrimaryExpression::Float(value) => writeln!(f, "{prefix}float: {value}"),
            PrimaryExpression::String(value) => writeln!(f, "{prefix}string: \"{value}\""),
            PrimaryExpression::Identifier(value) => writeln!(f, "{prefix}ident: {value}"),
            PrimaryExpression::Expr(node) => node.as_ref().ast_fmt(f, prefix),
        }
    }
}

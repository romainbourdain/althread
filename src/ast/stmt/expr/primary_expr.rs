use std::{
    fmt::{self, Debug},
    str::FromStr,
};

use pest::iterators::Pair;

use crate::{
    ast::{
        display::AstDisplay,
        node::{Build, Node},
        token::identifier::Identifier,
    },
    error::{AlthreadError, AlthreadResult, ErrorType},
    no_rule,
    parser::Rule,
};

use super::Expr;

#[derive(Debug)]
pub enum PrimaryExpr {
    Null(),
    Bool(bool),
    Int(i64),
    Float(f64),
    String(String),
    Identifier(Identifier),
    Expr(Box<Node<Expr>>),
}

impl PrimaryExpr {
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
                Rule::BOOLEAN => Self::Bool(parse_with_error::<bool>(pair)?),
                Rule::INTEGER => Self::Int(parse_with_error::<i64>(pair)?),
                Rule::FLOAT => Self::Float(parse_with_error::<f64>(pair)?),
                Rule::STRING => Self::String(pair.as_str().to_string()),
                Rule::IDENTIFIER => Self::Identifier(Identifier::build(pair.into_inner())?),
                Rule::expr => Self::Expr(Box::new(Node::build(pair)?)),
                _ => return Err(no_rule!(pair)),
            },
        })
    }
}

impl AstDisplay for PrimaryExpr {
    fn ast_fmt(&self, f: &mut fmt::Formatter, prefix: &crate::ast::display::Prefix) -> fmt::Result {
        match self {
            PrimaryExpr::Null() => writeln!(f, "{prefix}null"),
            PrimaryExpr::Bool(value) => writeln!(f, "{prefix}bool: {value}"),
            PrimaryExpr::Int(value) => writeln!(f, "{prefix}int: {value}"),
            PrimaryExpr::Float(value) => writeln!(f, "{prefix}float: {value}"),
            PrimaryExpr::String(value) => writeln!(f, "{prefix}string: \"{value}\""),
            PrimaryExpr::Identifier(value) => writeln!(f, "{prefix}ident: {value}"),
            PrimaryExpr::Expr(node) => node.as_ref().ast_fmt(f, prefix),
        }
    }
}

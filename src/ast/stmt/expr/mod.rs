pub mod binary_expr;
pub mod unary_expr;

use std::fmt;

use binary_expr::BinaryExpr;
use pest::iterators::Pair;
use unary_expr::UnaryExpr;

use crate::{
    ast::{
        display::{AstDisplay, Prefix},
        node::{Build, Node},
        token::literals::Literal,
    },
    error::AlthreadResult,
    no_rule,
    parser::Rule,
};

#[derive(Debug)]
pub enum Expr {
    Binary(Node<BinaryExpr>),
    Unary(Node<UnaryExpr>),
    Primary(Node<Literal>),
}

impl Build for Expr {
    fn build(pair: Pair<Rule>) -> AlthreadResult<Self> {
        let nb_pairs = pair.clone().into_inner().count();

        match pair.as_rule() {
            Rule::expr => Expr::build(pair.into_inner().next().unwrap()),

            Rule::primary => Ok(Self::Primary(Node::build(
                pair.into_inner().next().unwrap(),
            )?)),

            Rule::logical_or
            | Rule::logical_and
            | Rule::equality
            | Rule::comparison
            | Rule::term
            | Rule::factor => {
                let (line, column) = pair.line_col();
                let mut pairs = pair.into_inner();

                let mut left_expr = Expr::build(pairs.next().unwrap())?;

                while let Some(operator_pair) = pairs.next() {
                    let operator = Node::build(operator_pair)?;

                    let right_pair = pairs.next().unwrap();
                    let (right_line, right_column) = right_pair.line_col();
                    let right_expr = Expr::build(right_pair)?;

                    left_expr = Expr::Binary(Node {
                        value: BinaryExpr {
                            left: Box::new(Node {
                                value: left_expr,
                                line,
                                column,
                            }),
                            operator,
                            right: Box::new(Node {
                                value: right_expr,
                                line: right_line,
                                column: right_column,
                            }),
                        },
                        line,
                        column,
                    });
                }

                Ok(left_expr)
            }

            Rule::unary => {
                if nb_pairs == 1 {
                    Expr::build(pair.into_inner().next().unwrap())
                } else {
                    Ok(Self::Unary(Node::build(pair)?))
                }
            }
            _ => Err(no_rule!(pair)),
        }
    }
}

impl AstDisplay for Expr {
    fn ast_fmt(&self, f: &mut fmt::Formatter, prefix: &Prefix) -> fmt::Result {
        match self {
            Self::Binary(node) => node.ast_fmt(f, prefix),
            Self::Unary(node) => node.ast_fmt(f, prefix),
            Self::Primary(node) => writeln!(f, "{}literal: {}", prefix, node.value),
        }
    }
}

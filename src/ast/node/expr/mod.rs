pub mod binary_expr;
pub mod primary_expr;
pub mod unary_expr;

use binary_expr::BinaryExpr;
use pest::iterators::Pair;
use primary_expr::PrimaryExpr;
use unary_expr::UnaryExpr;

use crate::{ast::token::Token, error::AlthreadResult, parser::Rule};

#[derive(Debug)]
pub enum Expr {
    Binary(BinaryExpr),
    Unary(UnaryExpr),
    Primary(Token<PrimaryExpr>),
}

impl Expr {
    pub fn build(pair: Pair<Rule>) -> AlthreadResult<Self> {
        Ok(Self::Primary(Token {
            value: PrimaryExpr::Null(),
            line: 0,
            column: 0,
        }))
    }
}

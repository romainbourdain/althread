pub mod binary;
pub mod primary;
pub mod unary;

use binary::BinExpr;
use primary::PrimaryExpr;
use unary::UnExpr;

#[derive(Debug)]
pub struct Expr {
    pub kind: ExprKind,
    pub line: usize,
    pub column: usize,
}

impl Expr {
    pub fn new(kind: ExprKind) -> Self {
        Self {
            kind,
            line: 0,
            column: 0,
        }
    }
}

#[derive(Debug)]
pub enum ExprKind {
    Primary(PrimaryExpr),
    Binary(BinExpr),
    Unary(UnExpr),
}

impl ExprKind {
    pub fn new() -> Self {
        Self::Primary(PrimaryExpr::Null)
    }
}

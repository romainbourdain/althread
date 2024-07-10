pub mod binary;
pub mod primary;
pub mod unary;

use binary::BinExpr;
use primary::PrimaryExpr;
use unary::UnExpr;

use super::datatype::DataType;

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

    pub fn from_datatype(datatype: &DataType) -> Self {
        use DataType::*;

        let primary = match datatype {
            Int => PrimaryExpr::Int(0),
            Float => PrimaryExpr::Float(0.0),
            Bool => PrimaryExpr::Bool(false),
            String => PrimaryExpr::String("".to_string()),
            Void => PrimaryExpr::Null,
        };

        Self::new(ExprKind::Primary(primary))
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

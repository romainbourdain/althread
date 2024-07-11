use core::fmt;
use std::{fmt::Debug, ops::Sub};

use super::expr::{primary::PrimaryExpr, Expr, ExprKind};

#[derive(Debug)]
pub struct Assign {
    pub identifier: String,
    pub op: AssignOp,
    pub value: Expr,
    pub line: usize,
    pub column: usize,
}

impl Assign {
    pub fn new(line: usize, column: usize) -> Self {
        Self {
            identifier: "".to_string(),
            op: AssignOp::Assign,
            value: Expr::new(ExprKind::Primary(PrimaryExpr::Null)),
            line,
            column,
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum AssignOp {
    Assign,
    AddAssign,
    SubAssign,
    MulAssign,
    DivAssign,
    ModAssign,
}

impl fmt::Display for AssignOp {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use AssignOp::*;
        let op = match self {
            Assign => "=",
            AddAssign => "+=",
            SubAssign => "-=",
            MulAssign => "*=",
            DivAssign => "/=",
            ModAssign => "%=",
        };
        write!(f, "{}", op)
    }
}

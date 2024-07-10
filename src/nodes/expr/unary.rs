use core::fmt;

use super::Expr;

#[derive(Debug)]
pub struct UnExpr {
    pub op: UnOp,
    pub rhs: Box<Expr>,
}

#[derive(Debug, Clone)]
pub enum UnOp {
    Not,
    Neg,
}

impl fmt::Display for UnOp {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use UnOp::*;
        let op = match self {
            Not => "!",
            Neg => "-",
        };
        write!(f, "{}", op)
    }
}

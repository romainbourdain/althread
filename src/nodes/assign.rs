use super::expr::{primary::PrimaryExpr, Expr, ExprKind};

#[derive(Debug)]
pub struct Assign {
    pub identifier: String,
    pub op: AssignBinOp,
    pub value: Expr,
    pub line: usize,
    pub column: usize,
}

impl Assign {
    pub fn new(line: usize, column: usize) -> Self {
        Self {
            identifier: "".to_string(),
            op: AssignBinOp::Assign,
            value: Expr::new(ExprKind::Primary(PrimaryExpr::Null)),
            line,
            column,
        }
    }
}

#[derive(Debug)]
pub enum AssignBinOp {
    Assign,
    AddAssign,
    SubAssign,
    MulAssign,
    DivAssign,
    ModAssign,
}

use super::{
    datatype::DataType,
    expr::{primary::PrimaryExpr, Expr, ExprKind},
};

#[derive(Debug)]
pub struct Decl {
    pub identifier: String,
    pub value: Expr,
    pub datatype: DataType,
    pub mutable: bool,
    pub line: usize,
    pub column: usize,
}

impl Decl {
    pub fn new(line: usize, column: usize) -> Self {
        Self {
            identifier: "".to_string(),
            value: Expr::new(ExprKind::Primary(PrimaryExpr::Null)),
            datatype: DataType::new(),
            mutable: false,
            line,
            column,
        }
    }
}

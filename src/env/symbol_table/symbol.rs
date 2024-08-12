use crate::ast::{stmt::expr::primary_expr::PrimaryExpr, token::datatype::DataType};

#[derive(Debug)]
pub struct Symbol {
    pub mutable: bool,
    pub datatype: DataType,
    pub value: PrimaryExpr,
}

impl Symbol {
    pub fn new(mutable: bool, datatype: DataType, value: PrimaryExpr) -> Self {
        Self {
            mutable,
            datatype,
            value,
        }
    }
}

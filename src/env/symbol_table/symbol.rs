use crate::ast::{
    statement::expression::primary_expression::PrimaryExpression, token::datatype::DataType,
};

#[derive(Debug)]
pub struct Symbol {
    pub mutable: bool,
    pub datatype: DataType,
    pub value: PrimaryExpression,
}

impl Symbol {
    pub fn new(mutable: bool, datatype: DataType, value: PrimaryExpression) -> Self {
        Self {
            mutable,
            datatype,
            value,
        }
    }
}

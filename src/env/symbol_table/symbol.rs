use crate::ast::token::{datatype::DataType, literals::Literal};

#[derive(Debug)]
pub struct Symbol {
    pub mutable: bool,
    pub datatype: DataType,
    pub value: Literal,
}

impl Symbol {
    pub fn new(mutable: bool, datatype: DataType, value: Literal) -> Self {
        Self {
            mutable,
            datatype,
            value,
        }
    }
}

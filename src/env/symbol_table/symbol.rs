use crate::ast::token::{datatype::DataType, literal::Literal};

#[derive(Debug, Clone)]
pub struct Symbol {
    pub mutable: bool,
    pub datatype: DataType,
    pub value: Literal,
}

impl Symbol {
    pub fn new(mutable: bool, datatype: DataType, value: Literal) -> Result<Self, String> {
        Ok(Self {
            mutable,
            datatype,
            value,
        })
    }
}

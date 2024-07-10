use std::collections::HashMap;

use crate::nodes::{datatype::DataType, expr::primary::PrimaryExpr};

pub type SymbolTable = HashMap<String, Symbol>;

#[derive(Debug)]
pub struct Symbol {
    pub datatype: DataType,
    pub mutable: bool,
    pub value: Option<PrimaryExpr>,
}

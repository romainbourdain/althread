use std::collections::HashMap;

use crate::ast::{expr::primary::PrimaryExpr, token::datatype::DataType};

pub type SymbolTable = HashMap<String, Symbol>;

#[derive(Debug)]
pub struct Symbol {
    pub datatype: DataType,
    pub mutable: bool,
    pub value: Option<PrimaryExpr>,
}

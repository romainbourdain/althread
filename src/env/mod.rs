pub mod datatype;
pub mod display;
pub mod symbol_table;
pub mod value;

use std::collections::HashMap;

use datatype::DataType;
use value::Value;

#[derive(Debug)]
pub struct Environment<'a> {
    pub symbol_tables: Vec<SymbolTable>,
    pub global_table: &'a mut SymbolTable,
}

pub type SymbolTable = HashMap<String, Symbol>;

#[derive(Debug)]
pub struct Symbol {
    pub datatype: DataType,
    pub mutable: bool,
    pub value: Option<Value>,
}

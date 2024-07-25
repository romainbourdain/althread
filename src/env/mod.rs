pub mod datatype;
pub mod display;
pub mod symbol_table;

use std::collections::HashMap;

use datatype::DataType;

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
    pub value: Option<SymbolValue>,
}

#[derive(Debug)]
pub enum SymbolValue {
    Null,
    Bool(bool),
    Int(i64),
    Float(f64),
    String(String),
}

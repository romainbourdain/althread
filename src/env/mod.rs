pub mod datatype;
pub mod display;
pub mod symbol;
pub mod symbol_table;
pub mod value;

use symbol::Symbol;
use symbol_table::SymbolTable;

#[derive(Debug)]
pub struct Environment<'a> {
    pub symbol_tables: Vec<SymbolTable>,
    pub global_table: &'a mut SymbolTable,
}

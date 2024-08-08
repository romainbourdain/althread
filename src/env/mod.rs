pub mod datatype;
// pub mod display;
pub mod symbol;
pub mod symbol_table;
pub mod value;

use symbol::Symbol;
use symbol_table::SingleSymbolTable;

#[derive(Debug)]
pub struct Environment {
    pub global_table: SingleSymbolTable,
    pub running_process: Vec<String>,
    pub process_table: SingleSymbolTable,
}

impl Environment {
    pub fn new() -> Self {
        Self {
            global_table: SingleSymbolTable::new(),
            running_process: Vec::new(),
            process_table: SingleSymbolTable::new(),
        }
    }
}

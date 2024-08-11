use std::{cell::RefCell, rc::Rc};

use super::SymbolTable;

#[derive(Debug)]
pub struct SymbolTableStack {
    pub global: Rc<RefCell<SymbolTable>>,
    pub tables: Vec<SymbolTable>,
}

impl SymbolTableStack {
    pub fn new(global: Rc<RefCell<SymbolTable>>) -> Self {
        Self {
            global,
            tables: Vec::new(),
        }
    }
}

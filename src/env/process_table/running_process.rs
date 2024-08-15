use std::{cell::RefCell, collections::HashMap, rc::Rc};

use crate::env::symbol_table::{symbol_table_stack::SymbolTableStack, SymbolTable};

use super::process::Process;

#[derive(Debug)]
pub struct RunningProcess {
    pub processes: HashMap<String, Process>,
}

impl RunningProcess {
    pub fn new() -> Self {
        Self {
            processes: HashMap::new(),
        }
    }

    pub fn insert(&mut self, identifier: String, global_table: &Rc<RefCell<SymbolTable>>) {
        let symbol_table = Rc::new(RefCell::new(SymbolTableStack::new(global_table)));
        self.processes
            .insert(identifier, Process::new(&symbol_table));
    }
}

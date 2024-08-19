use std::{cell::RefCell, rc::Rc};

use crate::env::symbol_table::{process_table::ProcessTable, symbol_table_stack::SymbolTableStack};

use super::Env;

#[derive(Debug)]
pub struct ProcessEnv {
    pub symbol_table: Rc<RefCell<SymbolTableStack>>, // the process symbol table stack
    pub process_table: Rc<RefCell<ProcessTable>>,
    pub position: usize,                // the current position in the AST
    pub child: Option<Box<ProcessEnv>>, // the child scope environment
}

impl ProcessEnv {
    pub fn new(
        symbol_table: &Rc<RefCell<SymbolTableStack>>,
        process_table: &Rc<RefCell<ProcessTable>>,
    ) -> Self {
        Self {
            position: 0,
            child: None,
            symbol_table: Rc::clone(symbol_table),
            process_table: Rc::clone(process_table),
        }
    }

    pub fn new_child(&self) -> Self {
        Self {
            position: 0,
            child: None,
            symbol_table: Rc::clone(&self.symbol_table),
            process_table: Rc::clone(&self.process_table),
        }
    }

    pub fn new_global(env: &Env) -> Self {
        Self {
            position: 0,
            child: None,
            symbol_table: Rc::new(RefCell::new(SymbolTableStack::new(&env.global_table))),
            process_table: Rc::clone(&env.process_table),
        }
    }

    pub fn consume(&mut self) {
        self.clean();
        self.position += 1;
    }

    pub fn reset(&mut self) {
        self.clean();
        self.position = 0;
    }

    pub fn clean(&mut self) {
        self.child = None;
    }

    pub fn get_child(&mut self) -> &mut ProcessEnv {
        if self.child.is_none() {
            self.child = Some(Box::new(self.new_child()));
        }

        self.child.as_mut().unwrap()
    }
}

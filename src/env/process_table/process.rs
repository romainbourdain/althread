use std::{cell::RefCell, rc::Rc};

use crate::env::symbol_table::{symbol_table_stack::SymbolTableStack, SymbolTable};

use super::{running_process::RunningProcess, ProcessTable};

#[derive(Debug)]
pub struct Process {
    pub symbol_table: SymbolTableStack,
    pub process_table: Rc<RefCell<ProcessTable>>,
    pub running_process: Rc<RefCell<RunningProcess>>,
}

impl Process {
    pub fn new(
        global_table: &Rc<RefCell<SymbolTable>>,
        process_table: &Rc<RefCell<ProcessTable>>,
        running_process: &Rc<RefCell<RunningProcess>>,
    ) -> Self {
        Self {
            symbol_table: SymbolTableStack::new(&global_table),
            process_table: Rc::clone(process_table),
            running_process: Rc::clone(running_process),
        }
    }
}

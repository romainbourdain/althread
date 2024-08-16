use std::{cell::RefCell, rc::Rc};

use crate::env::symbol_table::{symbol_table_stack::SymbolTableStack, SymbolTable};

use super::{process_env::ProcessEnv, ProcessTable};

#[derive(Debug)]
pub struct RunningProcesses {
    pub processes: Vec<RunningProcess>,
}

impl RunningProcesses {
    pub fn new() -> Self {
        Self {
            processes: Vec::new(),
        }
    }

    pub fn insert(
        &mut self,
        identifier: String,
        process_table: &Rc<RefCell<ProcessTable>>,
    ) -> Result<(), String> {
        if !process_table.borrow().processes.contains(&identifier) {
            return Err(format!("Process {} not found", identifier));
        }

        self.processes.push(RunningProcess::new(identifier));

        Ok(())
    }
}

#[derive(Debug)]
pub struct RunningProcess {
    pub name: String,
    pub process: Option<ProcessEnv>,
}

impl RunningProcess {
    pub fn new(name: String) -> Self {
        Self {
            name,
            process: None,
        }
    }
}

use std::{cell::RefCell, rc::Rc};

use crate::env::{symbol_table::symbol_table_stack::SymbolTableStack, Env};

use super::process_env::ProcessEnv;

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

    pub fn insert(&mut self, identifier: String, env: &Env) -> Result<(), String> {
        if !env.process_table.borrow().processes.contains(&identifier) {
            return Err(format!("Process {} not found", identifier));
        }

        self.processes.push(RunningProcess::new(identifier, env));

        Ok(())
    }
}

#[derive(Debug)]
pub struct RunningProcess {
    pub name: String,
    pub process: ProcessEnv,
}

impl RunningProcess {
    pub fn new(name: String, env: &Env) -> Self {
        Self {
            name,
            process: ProcessEnv::new(
                &Rc::new(RefCell::new(SymbolTableStack::new(&env.global_table))),
                &env.process_table,
            ),
        }
    }
}

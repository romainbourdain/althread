use std::{cell::RefCell, rc::Rc};

use crate::{
    ast::{node::Node, statement::expression::Expression},
    env::{symbol_table::symbol_table_stack::SymbolTableStack, Env},
};

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

    pub fn insert(&mut self, identifier: String, id: usize, env: &Env) -> Result<(), String> {
        if !env.process_table.borrow().processes.contains(&identifier) {
            return Err(format!("Process {} not found", identifier));
        }

        self.processes
            .push(RunningProcess::new(identifier, id, env));

        Ok(())
    }
}

#[derive(Debug)]
pub struct RunningProcess {
    pub name: String,
    pub process: ProcessEnv,
    pub condition: Option<Node<Expression>>,
}

impl RunningProcess {
    pub fn new(name: String, id: usize, env: &Env) -> Self {
        Self {
            name,
            process: ProcessEnv::new(
                id,
                &Rc::new(RefCell::new(SymbolTableStack::new(&env.global_table))),
                &env.process_table,
            ),
            condition: None,
        }
    }
}

pub mod process_table;
pub mod symbol_table;

use std::{cell::RefCell, rc::Rc};

use process_table::{running_process::RunningProcesses, ProcessTable};
use rand::{seq::IteratorRandom, thread_rng};
use symbol_table::SymbolTable;

use crate::{
    ast::Ast,
    error::{AlthreadError, AlthreadResult, ErrorType},
};

#[derive(Debug)]
pub struct Env {
    pub process_table: Rc<RefCell<ProcessTable>>, // contains all the process declared one time
    pub running_process: Rc<RefCell<RunningProcesses>>, // contains the process that are currently running (can be similar)
    pub global_table: Rc<RefCell<SymbolTable>>,
}

impl Env {
    pub fn new() -> Self {
        Self {
            global_table: Rc::new(RefCell::new(SymbolTable::new())),
            process_table: Rc::new(RefCell::new(ProcessTable::new())),
            running_process: Rc::new(RefCell::new(RunningProcesses::new())),
        }
    }

    pub fn run(&mut self, ast: &Ast) -> AlthreadResult<()> {
        for (name, _) in &ast.process_blocks {
            self.process_table.borrow_mut().push(name.clone());
        }

        self.running_process
            .borrow_mut()
            .insert("main".to_string(), &self)
            .map_err(|_| {
                AlthreadError::new(
                    ErrorType::SyntaxError,
                    1,
                    1,
                    "Program requires a main process".to_string(),
                )
            })?;

        // main loop : at each iteration, we choose a random process in self.running_process and we consume it
        loop {
            // choose a random process in self.running_process
            let process_index = {
                let running_process = self.running_process.borrow();
                if running_process.processes.is_empty() {
                    break;
                }

                let mut rng = thread_rng();
                (0..running_process.processes.len())
                    .choose(&mut rng)
                    .unwrap()
            };

            // get the random process
            let mut running_processes = self.running_process.borrow_mut();
            let running_process = &mut running_processes.processes[process_index];

            // consume the process
            if ast
                .eval(running_process.name.clone(), &mut running_process.process)?
                .is_some()
            {
                running_processes.processes.remove(process_index);
                if running_processes.processes.is_empty() {
                    break;
                }
            }
        }

        Ok(())
    }
}

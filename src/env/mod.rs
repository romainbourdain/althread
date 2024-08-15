pub mod process_table;
pub mod symbol_table;

use std::{cell::RefCell, rc::Rc};

use process_table::{running_process::RunningProcess, ProcessTable};
use rand::{seq::IteratorRandom, thread_rng};
use symbol_table::SymbolTable;

use crate::{ast::Ast, error::AlthreadResult};

#[derive(Debug)]
pub struct Env {
    pub process_table: Rc<RefCell<ProcessTable>>, // contains all the process declared one time
    pub running_process: Rc<RefCell<RunningProcess>>, // contains the process that are currently running (can be similar)
    pub global_table: Rc<RefCell<SymbolTable>>,
}

impl Env {
    pub fn new() -> Self {
        Self {
            global_table: Rc::new(RefCell::new(SymbolTable::new())),
            process_table: Rc::new(RefCell::new(ProcessTable::new())),
            running_process: Rc::new(RefCell::new(RunningProcess::new())),
        }
    }

    pub fn run(&mut self, ast: &Ast) -> AlthreadResult<()> {
        self.process_table.borrow_mut().push("main".to_string());

        self.running_process
            .borrow_mut()
            .insert("main".to_string(), &self);

        // main loop : at each iteration, we choose a random process in self.running_process and we consume it
        loop {
            // choose a random process in self.running_process
            let process_key = {
                let running_process = self.running_process.borrow();
                let keys = running_process.processes.keys().collect::<Vec<_>>();
                if keys.is_empty() {
                    break;
                }

                keys.into_iter().choose(&mut thread_rng()).unwrap().clone()
            };

            // get the random process
            let mut running_processes = self.running_process.borrow_mut();
            let process = running_processes.processes.get_mut(&process_key).unwrap();

            // consume the process
            if ast.eval(process_key.clone(), process)?.is_some() {
                break;
            }
        }

        Ok(())
    }
}

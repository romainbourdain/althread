pub mod process_table;
pub mod symbol_table;

use std::{cell::RefCell, rc::Rc};

use process_table::{running_process::RunningProcess, ProcessTable};
use symbol_table::SymbolTable;

use crate::{ast::Ast, error::AlthreadResult};

#[derive(Debug)]
pub struct Env {
    pub process_table: Rc<RefCell<ProcessTable>>,
    pub running_process: Rc<RefCell<RunningProcess>>,
    pub global_table: Rc<RefCell<SymbolTable>>,
}

impl Env {
    pub fn new() -> Self {
        let global_table = Rc::new(RefCell::new(SymbolTable::new()));
        let process_table = Rc::new(RefCell::new(ProcessTable::new()));
        let running_process = Rc::new(RefCell::new(RunningProcess::new()));

        Self {
            process_table,
            running_process,
            global_table,
        }
    }

    pub fn run(&mut self, ast: &Ast) -> AlthreadResult<()> {
        self.process_table.borrow_mut().push("main".to_string());

        self.running_process
            .borrow_mut()
            .insert("main".to_string(), &self.global_table);

        for (identifier, process) in self.running_process.borrow_mut().processes.iter_mut() {
            while ast.eval(identifier.clone(), process)?.is_none() {}
        }
        Ok(())
    }
}

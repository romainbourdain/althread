pub mod process_table;
pub mod symbol_table;

use std::{cell::RefCell, rc::Rc};

use process_table::{process::Process, running_process::RunningProcess, ProcessTable};
use symbol_table::SymbolTable;

use crate::ast::Ast;

#[derive(Debug)]
pub struct Env {
    pub process_table: Rc<RefCell<ProcessTable>>,
    pub global_table: Rc<RefCell<SymbolTable>>,
    pub running_process: Rc<RefCell<RunningProcess>>,
}

impl Env {
    pub fn new() -> Self {
        Self {
            process_table: Rc::new(RefCell::new(ProcessTable::new())),
            global_table: Rc::new(RefCell::new(SymbolTable::new())),
            running_process: Rc::new(RefCell::new(RunningProcess::new())),
        }
    }

    pub fn run(&mut self, ast: &Ast) {
        if let Some(global_brick) = &ast.global_brick {
            println!("Run global brick");
        }

        for (name, brick) in &ast.condition_bricks {
            println!("Run condition brick {}", name);
        }

        for (name, brick) in &ast.process_bricks {
            let process = Process::new(
                &self.global_table,
                &self.process_table,
                &self.running_process,
            );

            self.process_table
                .borrow_mut()
                .insert(name.clone(), process);
        }

        self.running_process
            .borrow_mut()
            .push("main".to_string(), &self.process_table);

        println!("{}", self.process_table.borrow());
        println!("{:?}", self.running_process.borrow());

        // TODO : Boucle principale
    }
}

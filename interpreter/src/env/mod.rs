pub mod process_env;
pub mod running_process;
pub mod symbol_table;

use std::{cell::RefCell, rc::Rc};

use rand::{rngs::StdRng, seq::IteratorRandom, SeedableRng};
use running_process::{RunningProcess, RunningProcesses};
use symbol_table::{process_table::ProcessTable, SymbolTable};

use crate::{
    args::Config,
    ast::Ast,
    error::{AlthreadError, AlthreadResult, ErrorType},
};

#[derive(Debug)]
pub struct Env {
    pub process_table: Rc<RefCell<ProcessTable>>,
    pub global_table: Rc<RefCell<SymbolTable>>,
}

impl Env {
    pub fn new() -> Self {
        Self {
            global_table: Rc::new(RefCell::new(SymbolTable::new())),
            process_table: Rc::new(RefCell::new(ProcessTable::new())),
        }
    }

    pub fn run(&mut self, ast: &Ast, config: &Config) -> AlthreadResult<()> {
        let mut rng = StdRng::seed_from_u64(config.seed);

        for (name, _) in &ast.process_blocks {
            self.process_table.borrow_mut().push(name.to_string());
        }

        self.process_table.borrow_mut().queue("main".to_string());

        ast.eval_globals(self)?;

        let mut running_processes = RunningProcesses::new();
        loop {
            if running_processes.processes.is_empty()
                && self.process_table.borrow().queue.is_empty()
            {
                break;
            }

            self.dequeue_process(&mut running_processes)?;

            ast.eval_conditions(self)?;

            let (chosen_process, process_index) =
                match self.choose_process(&mut running_processes, &mut rng) {
                    Some(res) => res,
                    None => break,
                };

            if ast
                .eval_process(chosen_process.name.to_string(), &mut chosen_process.process)?
                .is_some()
            {
                running_processes.processes.remove(process_index);
            }
        }

        Ok(())
    }

    pub fn dequeue_process(
        &mut self,
        running_processes: &mut RunningProcesses,
    ) -> AlthreadResult<()> {
        for process_name in &self.process_table.borrow().queue {
            running_processes
                .insert(process_name.to_string(), &self)
                .map_err(|e| AlthreadError::new(ErrorType::SyntaxError, 1, 1, e))?;
        }
        self.process_table.borrow_mut().queue.clear();
        Ok(())
    }

    pub fn choose_process<'a>(
        &self,
        running_processes: &'a mut RunningProcesses,
        rng: &mut StdRng,
    ) -> Option<(&'a mut RunningProcess, usize)> {
        let process_index = {
            if running_processes.processes.is_empty() {
                return None;
            }
            (0..running_processes.processes.len()).choose(rng).unwrap()
        };

        Some((
            &mut running_processes.processes[process_index],
            process_index,
        ))
    }
}

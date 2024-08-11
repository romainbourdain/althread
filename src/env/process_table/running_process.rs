use std::{cell::RefCell, rc::Rc};

use super::ProcessTable;

#[derive(Debug)]
pub struct RunningProcess {
    pub processes: Vec<String>,
}

impl RunningProcess {
    pub fn new() -> Self {
        Self {
            processes: Vec::new(),
        }
    }

    pub fn push(&mut self, process: String, process_table: &Rc<RefCell<ProcessTable>>) {
        let process_table = Rc::clone(process_table);
        if process_table.borrow().processes.contains_key(&process) {
            self.processes.push(process);
        } else {
            panic!("Process {} doesn't exist", process);
        }
    }
}

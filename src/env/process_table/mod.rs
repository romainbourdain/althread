pub mod process;
pub mod running_process;

use std::collections::HashMap;

use process::Process;

#[derive(Debug)]
pub struct ProcessTable {
    pub processes: HashMap<String, Process>,
}

impl ProcessTable {
    pub fn new() -> Self {
        Self {
            processes: HashMap::new(),
        }
    }

    pub fn insert(&mut self, name: String, process: Process) {
        self.processes.insert(name, process);
    }
}

impl std::fmt::Display for ProcessTable {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        for (name, _) in &self.processes {
            writeln!(f, "{}", name)?;
        }

        Ok(())
    }
}

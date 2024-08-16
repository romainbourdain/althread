pub mod process;
pub mod running_process;

#[derive(Debug)]
pub struct ProcessTable {
    pub processes: Vec<String>,
}

impl ProcessTable {
    pub fn new() -> Self {
        Self {
            processes: Vec::new(),
        }
    }

    pub fn push(&mut self, name: String) {
        self.processes.push(name);
    }
}

impl std::fmt::Display for ProcessTable {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        for name in &self.processes {
            writeln!(f, "{}", name)?;
        }

        Ok(())
    }
}

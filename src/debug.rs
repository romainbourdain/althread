use std::{
    io::{self, Write},
    process::Command,
};

use crate::env::Environment;

pub struct Debug {
    code_history: Vec<String>,
}

impl Debug {
    pub fn new() -> Self {
        Self {
            code_history: Vec::new(),
        }
    }

    pub fn push(&mut self, line: String) {
        self.code_history.push(line);
    }

    pub fn prompt_user(&self, env: &Environment) {
        Self::clear_terminal();
        for line in &self.code_history {
            println!("{}", line);
        }

        loop {
            match DebugCommand::get() {
                DebugCommand::Step => break,
                DebugCommand::Env => println!("{}", env),
                DebugCommand::Exit => return,
            }
        }
    }

    fn clear_terminal() {
        if cfg!(target_os = "windows") {
            Command::new("cmd").args(&["/C", "cls"]).status().unwrap();
        } else {
            Command::new("clear").status().unwrap();
        }
    }
}

enum DebugCommand {
    Step,
    Env,
    Exit,
}

impl DebugCommand {
    pub fn get() -> Self {
        loop {
            io::stdout().flush().unwrap();

            let mut input = String::new();
            io::stdin().read_line(&mut input).unwrap();
            let input = input.trim();

            match input.to_lowercase().as_str() {
                "step" | "" => return DebugCommand::Step,
                "env" => return DebugCommand::Env,
                "exit" => return DebugCommand::Exit,
                _ => println!("Invalid command. Please enter 'step', 'env' or 'exit'."),
            }
        }
    }
}

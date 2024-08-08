pub mod atomic;
pub mod block;
pub mod display;
pub mod node;
pub mod process;

use std::collections::HashMap;

use pest::iterators::Pairs;
use process::Process;

use crate::{env::Environment, error::AlthreadResult, no_rule, parser::Rule};

#[derive(Debug)]
pub struct Ast<'a> {
    pub process_bricks: HashMap<String, Process<'a>>,
    pub condition_bricks: Vec<Process<'a>>,
    pub global_bricks: Vec<Process<'a>>,
    pub environment: Environment,
}

impl<'a> Ast<'a> {
    pub fn new() -> Self {
        Self {
            process_bricks: HashMap::new(),
            condition_bricks: Vec::new(),
            global_bricks: Vec::new(),
            environment: Environment::new(),
        }
    }

    pub fn build(pairs: Pairs<'a, Rule>) -> AlthreadResult<Self> {
        let mut ast = Self::new();

        for pair in pairs {
            let rule = pair.as_rule();
            match rule {
                Rule::main_brick => {
                    let mut main_process = Process::build(pair.into_inner())?;
                    main_process.symbol_table.push();
                    ast.process_bricks.insert("main".to_string(), main_process);
                }

                Rule::process_brick => {
                    let mut pairs = pair.into_inner();
                    let name_pair = pairs.next().unwrap();

                    let mut process = Process::build(pairs)?;
                    process.symbol_table.push();
                    ast.process_bricks
                        .insert(name_pair.as_str().to_string(), process);
                }

                Rule::cond_brick => ast
                    .condition_bricks
                    .push(Process::build(pair.into_inner())?),

                Rule::global_brick => ast.global_bricks.push(Process::build(pair.into_inner())?),

                Rule::EOI => (),
                _ => return Err(no_rule!(pair)),
            }
        }

        Ok(ast)
    }
}

impl Ast<'_> {
    pub fn eval(&mut self) -> AlthreadResult<()> {
        for brick in &mut self.global_bricks {
            brick.consume(&mut self.environment)?;
        }
        for brick in &mut self.condition_bricks {
            brick.consume(&mut self.environment)?;
        }
        for (_, brick) in &mut self.process_bricks {
            loop {
                /*                 io::stdout().flush().expect("Erreur de flush");
                let mut input = String::new();
                io::stdin()
                    .read_line(&mut input)
                    .expect("Erreur de lecture"); */

                if !brick.consume(&mut self.environment)? {
                    break;
                }
            }
        }
        Ok(())
    }
}

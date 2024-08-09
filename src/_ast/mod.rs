pub mod atomic;
pub mod block;
pub mod brick;
pub mod display;
pub mod node;

use std::collections::HashMap;

use pest::iterators::Pairs;

use crate::{env::Environment, error::AlthreadResult, no_rule, parser::Rule};
use rand::Rng;

#[derive(Debug)]
pub struct Ast<'a> {
    pub process_bricks: HashMap<String, Pairs<'a, Rule>>,
    pub condition_bricks: Vec<Pairs<'a, Rule>>,
    pub global_bricks: Vec<Pairs<'a>>,
    pub env: Environment<'a>,
}

impl<'a> Ast<'a> {
    pub fn new() -> Self {
        Self {
            process_bricks: HashMap::new(),
            condition_bricks: Vec::new(),
            global_bricks: Vec::new(),
            env: Environment::new(),
        }
    }

    pub fn build(pairs: Pairs<'a, Rule>) -> AlthreadResult<Self> {
        let mut ast = Self::new();

        for pair in pairs {
            let rule = pair.as_rule();
            match rule {
                Rule::main_brick => {
                    let mut main_process = Brick::build(pair.into_inner())?;
                    main_process.symbol_table.push();
                    ast.process_bricks.insert("main".to_string(), main_process);
                }

                Rule::process_brick => {
                    let mut pairs = pair.into_inner();
                    let name_pair = pairs.next().unwrap();

                    let mut process = Brick::build(pairs)?;
                    process.symbol_table.push();
                    ast.process_bricks
                        .insert(name_pair.as_str().to_string(), process);
                }

                Rule::cond_brick => ast.condition_bricks.push(Brick::build(pair.into_inner())?),

                Rule::global_brick => ast.global_bricks.push(Brick::build(pair.into_inner())?),

                Rule::EOI => (),
                _ => return Err(no_rule!(pair)),
            }
        }

        Ok(ast)
    }
}

impl Ast<'_> {
    pub fn run(&mut self) -> AlthreadResult<()> {
        println!("Running processes:");
        println!("{:?}\n", self.env.running_process);

        for brick in &mut self.global_bricks {
            brick.consume(&mut self.env)?;
        }
        for brick in &mut self.condition_bricks {
            brick.consume(&mut self.env)?;
        }

        self.env
            .run_process("main".to_string(), &self.process_bricks)?;

        loop {
            // get a random running process
        }

        for (_, brick) in &mut self.process_bricks {
            loop {
                /*                 io::stdout().flush().expect("Erreur de flush");
                let mut input = String::new();
                io::stdin()
                    .read_line(&mut input)
                    .expect("Erreur de lecture"); */

                if !brick.consume(&mut self.env)? {
                    break;
                }
            }
        }
        Ok(())
    }
}

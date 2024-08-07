pub mod atomic;
pub mod block;
pub mod brick;
pub mod display;
pub mod node;

use std::collections::HashMap;

use brick::Brick;
use pest::iterators::Pairs;

use crate::{args::Config, env::Environment, error::AlthreadResult, no_rule, parser::Rule};

#[derive(Debug)]
pub struct Ast<'a> {
    pub process_bricks: HashMap<String, Brick<'a>>,
    pub condition_bricks: Vec<Brick<'a>>,
    pub global_bricks: Vec<Brick<'a>>,
}

impl<'a> Ast<'a> {
    pub fn new() -> Self {
        Self {
            process_bricks: HashMap::new(),
            condition_bricks: Vec::new(),
            global_bricks: Vec::new(),
        }
    }

    pub fn build(pairs: Pairs<'a, Rule>) -> AlthreadResult<Self> {
        let mut ast = Self::new();

        for pair in pairs {
            let rule = pair.as_rule();
            match rule {
                Rule::main_brick => {
                    ast.process_bricks
                        .insert("main".to_string(), Brick::build(pair.into_inner())?);
                }
                Rule::process_brick => {
                    let mut pairs = pair.into_inner();
                    let name_pair = pairs.next().unwrap();
                    ast.process_bricks
                        .insert(name_pair.as_str().to_string(), Brick::build(pairs)?);
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
    pub fn eval(&mut self, env: &mut Environment, config: &Config) -> AlthreadResult<()> {
        for brick in &mut self.global_bricks {
            brick.consume(env, config)?;
        }
        for brick in &mut self.condition_bricks {
            brick.consume(env, config)?;
        }
        for (_, brick) in &mut self.process_bricks {
            loop {
                /*                 io::stdout().flush().expect("Erreur de flush");
                let mut input = String::new();
                io::stdin()
                    .read_line(&mut input)
                    .expect("Erreur de lecture"); */

                if !brick.consume(env, config)? {
                    break;
                }
            }
        }
        Ok(())
    }
}

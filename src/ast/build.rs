use std::collections::HashMap;

use pest::iterators::Pairs;

use crate::{error::AlthreadError, no_rule, parser::Rule};

use super::Ast;

impl<'a> Ast<'a> {
    pub fn new() -> Self {
        Self {
            process_bricks: HashMap::new(),
            condition_bricks: Vec::new(),
            global_bricks: Vec::new(),
        }
    }

    pub fn build(pairs: Pairs<'a, Rule>) -> Result<Self, AlthreadError> {
        let mut ast = Self::new();

        for pair in pairs {
            let rule = pair.as_rule();
            match rule {
                Rule::main_brick => {
                    ast.process_bricks
                        .insert("main".to_string(), pair.into_inner());
                }
                Rule::process_brick => {
                    let mut pairs = pair.into_inner();
                    let name_pair = pairs.next().unwrap();
                    ast.process_bricks
                        .insert(name_pair.as_str().to_string(), pairs);
                }
                Rule::cond_brick => ast.condition_bricks.push(pair.into_inner()),
                Rule::global_brick => ast.global_bricks.push(pair.into_inner()),
                Rule::EOI => (),
                _ => return Err(no_rule!(pair)),
            }
        }

        Ok(ast)
    }
}

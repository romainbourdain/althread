pub mod check;
pub mod display;
pub mod eval;
pub mod node;

use node::Node;
use pest::iterators::Pairs;
use std::collections::HashMap;

use crate::{error::AlthreadResult, no_rule, parser::Rule};

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

#[derive(Debug)]
pub struct Brick<'a> {
    pub nodes: Vec<Node<'a>>,
}

impl Brick<'_> {
    pub fn build<'a>(pairs: Pairs<'a, Rule>) -> AlthreadResult<Brick<'a>> {
        let mut nodes = Vec::new();
        for pair in pairs {
            nodes.push(Node::build(pair)?);
        }
        Ok(Brick { nodes })
    }
}

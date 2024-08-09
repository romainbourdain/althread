pub mod brick;
pub mod display;
pub mod node;
pub mod stmt;
pub mod token;

use std::{
    collections::HashMap,
    fmt::{self, Formatter},
};

use brick::Brick;
use display::AstDisplay;
use node::Node;
use pest::iterators::Pairs;

use crate::{error::AlthreadResult, no_rule, parser::Rule, write_indent};

#[derive(Debug)]
pub struct Ast {
    pub process_bricks: HashMap<String, Node<Brick>>,
    pub condition_bricks: HashMap<String, Node<Brick>>,
    pub global_brick: Option<Node<Brick>>,
}

impl Ast {
    pub fn new() -> Self {
        Self {
            process_bricks: HashMap::new(),
            condition_bricks: HashMap::new(),
            global_brick: None,
        }
    }

    pub fn build(pairs: Pairs<Rule>) -> AlthreadResult<Self> {
        let mut ast = Self::new();
        for pair in pairs {
            match pair.as_rule() {
                Rule::main_brick => {
                    let mut pairs = pair.into_inner();

                    let main_brick = Node::build(pairs.next().unwrap())?;
                    ast.process_bricks.insert("main".to_string(), main_brick);
                }
                Rule::global_brick => {
                    let mut pairs = pair.into_inner();

                    let global_brick = Node::build(pairs.next().unwrap())?;
                    ast.global_brick = Some(global_brick);
                }
                Rule::cond_brick => {
                    let mut pairs = pair.into_inner();

                    let cond_brick_key = pairs.next().unwrap().as_str().to_string();
                    let cond_brick = Node::build(pairs.next().unwrap())?;
                    ast.condition_bricks.insert(cond_brick_key, cond_brick);
                }
                Rule::process_brick => {
                    let mut pairs = pair.into_inner();

                    let process_brick_ident = pairs.next().unwrap().as_str().to_string();
                    let process_brick = Node::build(pairs.next().unwrap())?;
                    ast.process_bricks
                        .insert(process_brick_ident, process_brick);
                }
                Rule::EOI => (),
                _ => return Err(no_rule!(pair)),
            }
        }

        Ok(ast)
    }
}

impl fmt::Display for Ast {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        self.ast_fmt(f, 0)
    }
}

impl AstDisplay for Ast {
    fn ast_fmt(&self, f: &mut Formatter, indent_level: usize) -> fmt::Result {
        writeln!(f, "ast")?;
        if let Some(global_node) = &self.global_brick {
            write_indent!(f, indent_level, "shared")?;
            global_node.ast_fmt(f, indent_level + 1)?;
        }

        for (process_name, process_node) in &self.process_bricks {
            write_indent!(f, indent_level, "{}", process_name)?;
            process_node.ast_fmt(f, indent_level + 1)?;
        }

        Ok(())
    }
}

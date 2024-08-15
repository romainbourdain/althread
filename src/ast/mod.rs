pub mod block;
pub mod display;
pub mod node;
pub mod statement;
pub mod token;

use std::{
    collections::HashMap,
    fmt::{self, Formatter},
};

use block::Block;
use display::{AstDisplay, Prefix};
use node::Node;
use pest::iterators::Pairs;
use token::literal::Literal;

use crate::{
    env::process_table::process_env::ProcessEnv, error::AlthreadResult, no_rule, parser::Rule,
};

#[derive(Debug)]
pub struct Ast {
    pub process_blocks: HashMap<String, Node<Block>>,
    pub condition_blocks: HashMap<String, Node<Block>>,
    pub global_block: Option<Node<Block>>,
}

impl Ast {
    pub fn new() -> Self {
        Self {
            process_blocks: HashMap::new(),
            condition_blocks: HashMap::new(),
            global_block: None,
        }
    }

    pub fn build(pairs: Pairs<Rule>) -> AlthreadResult<Self> {
        let mut ast = Self::new();
        for pair in pairs {
            match pair.as_rule() {
                Rule::main_block => {
                    let mut pairs = pair.into_inner();

                    let main_block = Node::build(pairs.next().unwrap())?;
                    ast.process_blocks.insert("main".to_string(), main_block);
                }
                Rule::global_block => {
                    let mut pairs = pair.into_inner();

                    let global_block = Node::build(pairs.next().unwrap())?;
                    ast.global_block = Some(global_block);
                }
                Rule::condition_block => {
                    let mut pairs = pair.into_inner();

                    let condition_key = pairs.next().unwrap().as_str().to_string();
                    let condition_block = Node::build(pairs.next().unwrap())?;
                    ast.condition_blocks.insert(condition_key, condition_block);
                }
                Rule::process_block => {
                    let mut pairs = pair.into_inner();

                    let process_identifier = pairs.next().unwrap().as_str().to_string();
                    let process_block = Node::build(pairs.next().unwrap())?;
                    ast.process_blocks.insert(process_identifier, process_block);
                }
                Rule::EOI => (),
                _ => return Err(no_rule!(pair)),
            }
        }

        Ok(ast)
    }

    pub fn eval(
        &self,
        identifier: String,
        process: &mut ProcessEnv,
    ) -> AlthreadResult<Option<Literal>> {
        let block = self.process_blocks.get(&identifier).unwrap();
        block.eval(process)
    }
}

impl fmt::Display for Ast {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        self.ast_fmt(f, &Prefix::new())
    }
}

impl AstDisplay for Ast {
    fn ast_fmt(&self, f: &mut Formatter, prefix: &Prefix) -> fmt::Result {
        if let Some(global_node) = &self.global_block {
            writeln!(f, "{}shared", prefix)?;
            global_node.ast_fmt(f, &prefix.add_branch())?;
        }

        writeln!(f, "")?;

        for (condition_name, condition_node) in &self.condition_blocks {
            writeln!(f, "{}{}", prefix, condition_name)?;
            condition_node.ast_fmt(f, &prefix.add_branch())?;
            writeln!(f, "")?;
        }

        for (process_name, process_node) in &self.process_blocks {
            writeln!(f, "{}{}", prefix, process_name)?;
            process_node.ast_fmt(f, &prefix.add_branch())?;
            writeln!(f, "")?;
        }

        Ok(())
    }
}

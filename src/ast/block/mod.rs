pub mod if_block;
pub mod scope;
pub mod while_block;

use if_block::consume_if;
use pest::iterators::Pair;
use scope::consume_scope;
use while_block::consume_while;

use crate::{args::Config, env::Environment, error::AlthreadResult, parser::Rule};

use super::node::Node;

#[derive(Debug)]
pub struct Block<'a> {
    pub kind: BlockKind,
    pub pair: Pair<'a, Rule>,
    pub children: Vec<Node<'a>>,
    pub current: usize,
}

#[derive(Debug)]
pub enum BlockKind {
    Scope,
    If,
    While,
}

impl Block<'_> {
    pub fn new<'a>(pair: Pair<'a, Rule>) -> Block<'a> {
        Block {
            kind: match pair.as_rule() {
                Rule::scope => BlockKind::Scope,
                Rule::if_stmt => BlockKind::If,
                Rule::while_stmt => BlockKind::While,
                _ => panic!("Invalid block rule"),
            },
            pair,
            children: Vec::new(),
            current: 0,
        }
    }

    pub fn consume(&mut self, env: &mut Environment, config: &Config) -> AlthreadResult<bool> {
        match self.kind {
            BlockKind::Scope => Ok(consume_scope(self, env, config)?),
            BlockKind::If => Ok(consume_if(self, env, config)?),
            BlockKind::While => Ok(consume_while(self, env, config)?),
        }
    }

    pub fn reset(&mut self) {
        self.current = 0;
        for child in &mut self.children {
            child.reset();
        }
    }
}

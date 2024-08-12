use std::fmt;

use pest::iterators::Pairs;

use crate::{error::AlthreadResult, parser::Rule};

use super::{
    display::{AstDisplay, Prefix},
    node::{Build, Node},
    stmt::Stmt,
};

#[derive(Debug)]
pub struct Block {
    pub children: Vec<Node<Stmt>>,
}

impl Build for Block {
    fn build(pairs: Pairs<Rule>) -> AlthreadResult<Self> {
        let mut brick = Self::new();

        for pair in pairs {
            let node = Node::build(pair)?;
            brick.children.push(node);
        }

        Ok(brick)
    }
}

impl Block {
    pub fn new() -> Self {
        Self {
            children: Vec::new(),
        }
    }
}

impl AstDisplay for Block {
    fn ast_fmt(&self, f: &mut fmt::Formatter, prefix: &Prefix) -> fmt::Result {
        let mut node_count = self.children.len();
        for node in &self.children {
            node_count -= 1;
            if node_count == 0 {
                node.ast_fmt(f, &prefix.switch())?;
            } else {
                node.ast_fmt(f, &prefix)?;
            }
        }

        Ok(())
    }
}

use std::fmt;

use pest::iterators::Pair;

use crate::{error::AlthreadResult, parser::Rule};

use super::{
    display::AstDisplay, node::{Build, Node}, stmt::Stmt
};

#[derive(Debug)]
pub struct Brick {
    pub children: Vec<Node<Stmt>>,
}

impl Build for Brick {
    fn build(pair: Pair<Rule>) -> AlthreadResult<Self> {
        let mut brick = Self::new();

        for pair in pair.into_inner() {
            let node = Node::build(pair)?;
            brick.children.push(node);
        }

        Ok(brick)
    }
}

impl Brick {
    pub fn new() -> Self {
        Self {
            children: Vec::new(),
        }
    }
}

impl AstDisplay for Brick {
    fn ast_fmt(&self, f: &mut fmt::Formatter, indent_level: usize) -> fmt::Result {
        for node in &self.children {
            node.ast_fmt(f, indent_level)?;
        }

        Ok(())
    }
}

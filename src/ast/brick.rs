use std::fmt;

use pest::iterators::Pair;

use crate::{error::AlthreadResult, parser::Rule};

use super::{
    node::{Build, Node},
    stmt::Stmt,
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

impl fmt::Display for Brick {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for stmt in &self.children {
            writeln!(f, "{}", stmt)?;
        }

        Ok(())
    }
}

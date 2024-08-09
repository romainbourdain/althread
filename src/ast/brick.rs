use pest::iterators::Pair;

use crate::{error::AlthreadResult, parser::Rule};

use super::node::Node;

#[derive(Debug)]
pub struct Brick {
    pub children: Vec<Node>,
    pub line: usize,
    pub column: usize,
}

impl Brick {
    pub fn new((line, column): (usize, usize)) -> Self {
        Self {
            children: Vec::new(),
            line,
            column,
        }
    }
    pub fn build(pair: Pair<Rule>) -> AlthreadResult<Self> {
        let mut brick = Self::new(pair.line_col());

        for pair in pair.into_inner() {
            let node = Node::build(pair)?;
            brick.children.push(node);
        }

        Ok(brick)
    }
}

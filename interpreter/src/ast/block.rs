use std::fmt;

use pest::iterators::Pairs;

use crate::{
    env::{node_result::NodeResult, process_env::ProcessEnv},
    error::AlthreadResult,
    parser::Rule,
};

use super::{
    display::{AstDisplay, Prefix},
    node::{Node, NodeBuilder, NodeExecutor},
    statement::Statement,
};

#[derive(Debug)]
pub struct Block {
    pub children: Vec<Node<Statement>>,
}

impl NodeBuilder for Block {
    fn build(pairs: Pairs<Rule>) -> AlthreadResult<Self> {
        let mut block = Self::new();

        for pair in pairs {
            let node = Node::build(pair)?;
            block.children.push(node);
        }

        Ok(block)
    }
}

impl NodeExecutor for Block {
    fn eval(&self, env: &mut ProcessEnv) -> AlthreadResult<NodeResult> {
        if self.children.is_empty() {
            return Ok(NodeResult::null());
        }
        let node = &self.children[env.position];

        match node.eval(env.get_child())? {
            NodeResult::Finished(_) => env.consume(),
            NodeResult::Suspend(suspend) => return Ok(NodeResult::Suspend(suspend)),
            _ => {}
        }

        Ok(if env.position >= self.children.len() {
            NodeResult::null()
        } else {
            NodeResult::Incomplete
        })
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

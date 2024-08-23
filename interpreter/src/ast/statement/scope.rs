use std::fmt;

use pest::iterators::Pairs;

use crate::{
    ast::{
        display::{AstDisplay, Prefix},
        node::{Node, NodeBuilder, NodeExecutor},
        statement::Statement,
    },
    env::{node_result::NodeResult, process_env::ProcessEnv},
    error::AlthreadResult,
    parser::Rule,
};

#[derive(Debug)]
pub struct Scope {
    pub children: Vec<Node<Statement>>,
}

impl NodeBuilder for Scope {
    fn build(pairs: Pairs<Rule>) -> AlthreadResult<Self> {
        let children = pairs
            .map(Node::build)
            .collect::<AlthreadResult<Vec<Node<Statement>>>>()?;

        Ok(Self { children })
    }
}

impl NodeExecutor for Scope {
    fn eval(&self, env: &mut ProcessEnv) -> AlthreadResult<NodeResult> {
        if self.children.is_empty() {
            return Ok(NodeResult::null());
        }
        if env.position == 0 {
            env.symbol_table.borrow_mut().push();
        }

        let node = &self.children[env.position];
        match node.eval(env.get_child())? {
            NodeResult::Finished(_) => env.consume(),
            NodeResult::Suspend(suspend) => return Ok(NodeResult::Suspend(suspend)),
            _ => {}
        }

        Ok(if env.position >= self.children.len() {
            env.symbol_table.borrow_mut().pop();
            NodeResult::null()
        } else {
            NodeResult::Incomplete
        })
    }
}

impl AstDisplay for Scope {
    fn ast_fmt(&self, f: &mut fmt::Formatter, prefix: &Prefix) -> fmt::Result {
        writeln!(f, "{prefix}scope")?;
        let prefix = &prefix.add_branch();
        let mut child_count = self.children.len();
        for child in &self.children {
            child_count -= 1;
            if child_count == 0 {
                child.ast_fmt(f, &prefix.switch())?;
            } else {
                child.ast_fmt(f, prefix)?;
            }
        }

        Ok(())
    }
}

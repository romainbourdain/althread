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
pub struct AtomicScope {
    pub children: Vec<Node<Statement>>,
}

impl NodeBuilder for AtomicScope {
    fn build(pairs: Pairs<Rule>) -> AlthreadResult<Self> {
        let children = pairs
            .map(Node::build)
            .collect::<AlthreadResult<Vec<Node<Statement>>>>()?;

        Ok(Self { children })
    }
}

impl NodeExecutor for AtomicScope {
    fn eval(&self, env: &mut ProcessEnv) -> AlthreadResult<NodeResult> {
        if self.children.is_empty() {
            return Ok(NodeResult::null());
        }

        for child in &self.children {
            child.eval(env.get_child())?;
        }

        Ok(NodeResult::null())
    }
}

impl AstDisplay for AtomicScope {
    fn ast_fmt(&self, f: &mut fmt::Formatter, prefix: &Prefix) -> fmt::Result {
        writeln!(f, "{prefix}atomic")?;
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

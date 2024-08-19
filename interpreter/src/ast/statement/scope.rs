use std::fmt;

use pest::iterators::Pairs;

use crate::{
    ast::{
        display::{AstDisplay, Prefix},
        node::{Node, NodeBuilder, NodeExecutor},
        statement::Statement,
        token::literal::Literal,
    },
    env::process_env::ProcessEnv,
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
    fn eval(&self, env: &mut ProcessEnv) -> AlthreadResult<Option<Literal>> {
        let node = &self.children[env.position];
        if node.eval(env.get_child())?.is_some() {
            env.consume();
        }

        Ok((env.position >= self.children.len()).then(|| Literal::Null))
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

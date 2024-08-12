use std::fmt;

use pest::iterators::Pairs;

use crate::{
    ast::{
        display::{AstDisplay, Prefix},
        node::{Build, Node},
        stmt::Stmt,
    },
    error::AlthreadResult,
    parser::Rule,
};

#[derive(Debug)]
pub struct Scope {
    pub children: Vec<Node<Stmt>>,
}

impl Build for Scope {
    fn build(pairs: Pairs<Rule>) -> AlthreadResult<Self> {
        let children = pairs
            .map(Node::build)
            .collect::<AlthreadResult<Vec<Node<Stmt>>>>()?;

        Ok(Self { children })
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

use std::fmt;

use pest::iterators::Pair;

use crate::{
    ast::{
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
    fn build(pair: Pair<Rule>) -> AlthreadResult<Self> {
        let children = pair
            .into_inner()
            .map(Node::build)
            .collect::<AlthreadResult<Vec<Node<Stmt>>>>()?;

        Ok(Self { children })
    }
}

impl fmt::Display for Scope {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for stmt in &self.children {
            writeln!(f, "{}", stmt)?;
        }

        Ok(())
    }
}

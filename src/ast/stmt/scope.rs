use std::fmt;

use pest::iterators::Pair;

use crate::{
    ast::{
        display::AstDisplay,
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

impl AstDisplay for Scope {
    fn ast_fmt(&self, f: &mut fmt::Formatter, indent_level: usize) -> fmt::Result {
        for child in &self.children {
            child.ast_fmt(f, indent_level)?;
        }

        Ok(())
    }
}

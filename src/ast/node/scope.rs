use pest::iterators::Pair;

use crate::{ast::node::Node, error::AlthreadResult, parser::Rule};

#[derive(Debug)]
pub struct Scope {
    pub children: Vec<Node>,
    pub line: usize,
    pub column: usize,
}

impl Scope {
    pub fn build(pair: Pair<Rule>) -> AlthreadResult<Self> {
        let (line, column) = pair.line_col();
        let children = pair
            .into_inner()
            .map(Node::build)
            .collect::<AlthreadResult<Vec<Node>>>()?;

        Ok(Self {
            children,
            line,
            column,
        })
    }
}

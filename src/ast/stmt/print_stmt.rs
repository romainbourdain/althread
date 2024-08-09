use std::fmt;

use pest::iterators::Pair;

use crate::{
    ast::{
        display::{AstDisplay, Prefix},
        node::{Build, Node},
    },
    error::AlthreadResult,
    parser::Rule,
};

use super::expr::Expr;

#[derive(Debug)]
pub struct Print {
    pub value: Node<Expr>,
}

impl Build for Print {
    fn build(pair: Pair<Rule>) -> AlthreadResult<Self> {
        let mut pairs = pair.into_inner();

        let value = Node::build(pairs.next().unwrap())?;

        Ok(Self { value })
    }
}

impl AstDisplay for Print {
    fn ast_fmt(&self, f: &mut fmt::Formatter, prefix: &Prefix) -> fmt::Result {
        writeln!(f, "{prefix}print")?;
        self.value.ast_fmt(f, &prefix.add_leaf())?;

        Ok(())
    }
}

use pest::iterators::{Pair, Pairs};

use crate::{
    ast::node::Node,
    env::{symbol_table::SymbolTable, Environment},
    error::AlthreadResult,
    parser::Rule,
};

#[derive(Debug, Clone)]
pub struct Brick<'a> {
    pub nodes: Vec<Pair<'a>>,
    pub current: usize,
    pub symbol_table: SymbolTable,
}

impl Brick<'_> {
    pub fn build<'a>(pairs: Pairs<'a, Rule>) -> AlthreadResult<Brick<'a>> {
        let mut nodes = Vec::new();
        for pair in pairs {
            nodes.push(Node::build(pair)?);
        }
        Ok(Brick {
            nodes,
            current: 0,
            symbol_table: SymbolTable::new(),
        })
    }

    pub fn consume(&mut self, env: &mut Environment) -> AlthreadResult<bool> {
        if self.nodes.is_empty() {
            return Ok(false);
        }

        if !self.nodes[self.current].consume(&mut self.symbol_table, env)? {
            self.current += 1;
        }

        if self.current >= self.nodes.len() {
            return Ok(false);
        }

        Ok(true)
    }
}

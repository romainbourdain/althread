use pest::iterators::Pairs;

use crate::{
    ast::node::Node,
    env::{symbol_table::SymbolTable, Environment},
    error::AlthreadResult,
    parser::Rule,
};

#[derive(Debug)]
pub struct Process<'a> {
    pub nodes: Vec<Node<'a>>,
    pub current: usize,
    pub symbol_table: SymbolTable,
}

impl Process<'_> {
    pub fn build<'a>(pairs: Pairs<'a, Rule>) -> AlthreadResult<Process<'a>> {
        let mut nodes = Vec::new();
        for pair in pairs {
            nodes.push(Node::build(pair)?);
        }
        Ok(Process {
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

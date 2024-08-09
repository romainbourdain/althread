use pest::iterators::Pair;

use crate::{
    env::{symbol_table::SymbolTable, Environment},
    error::AlthreadResult,
    no_rule,
    parser::Rule,
};

use super::{atomic::Atomic, block::Block};

#[derive(Debug, Clone)]
pub enum Node<'a> {
    Atomic(Atomic<'a>),
    Block(Block<'a>),
}

impl Node<'_> {
    pub fn build(pair: Pair<Rule>) -> AlthreadResult<Node> {
        Ok(match pair.as_rule() {
            Rule::assignment | Rule::decl | Rule::expr | Rule::print_stmt | Rule::run_stmt => {
                Node::Atomic(Atomic::new(pair))
            }
            Rule::if_stmt | Rule::while_stmt | Rule::scope => {
                let mut block = Block::new(pair);
                for pair in block.pair.clone().into_inner() {
                    block.children.push(Self::build(pair)?);
                }
                Node::Block(block)
            }
            _ => return Err(no_rule!(pair)),
        })
    }

    pub fn consume(
        &mut self,
        symbol_table: &mut SymbolTable,
        env: &mut Environment,
    ) -> AlthreadResult<bool> {
        match self {
            Node::Atomic(atomic) => {
                atomic.consume(symbol_table, env)?;
                Ok(false)
            }
            Node::Block(block) => Ok(block.consume(symbol_table, env)?),
        }
    }

    pub fn reset(&mut self) {
        match self {
            Node::Block(block) => block.reset(),
            _ => (),
        }
    }
}

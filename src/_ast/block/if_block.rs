use crate::{
    ast::atomic::expr::consume_condition,
    env::{symbol_table::SymbolTable, Environment},
    error::AlthreadResult,
};

use super::Block;

pub fn consume_if(
    block: &mut Block,
    symbol_table: &mut SymbolTable,
    env: &mut Environment,
) -> AlthreadResult<bool> {
    if block.current == 0 {
        block.current = if consume_condition(&block.children[0], symbol_table, env)? {
            1
        } else {
            2
        };
        Ok(true)
    } else if block.current < block.children.len() {
        Ok(block.children[block.current].consume(symbol_table, env)?)
    } else {
        Ok(false)
    }
}

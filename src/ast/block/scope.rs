use crate::{
    env::{symbol_table::SymbolTable, Environment},
    error::AlthreadResult,
};

use super::Block;

pub fn consume_scope<'a>(
    block: &mut Block,
    symbol_table: &mut SymbolTable,
    env: &mut Environment,
) -> AlthreadResult<bool> {
    if block.children.is_empty() {
        return Ok(false);
    }

    if block.current == 0 {
        symbol_table.push();
    }

    if !block.children[block.current].consume(symbol_table, env)? {
        block.current += 1;
    }

    if block.current >= block.children.len() {
        symbol_table.pop();
        return Ok(false);
    }

    Ok(true)
}

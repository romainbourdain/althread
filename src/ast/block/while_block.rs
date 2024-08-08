use crate::{
    ast::atomic::expr::consume_condition,
    env::{symbol_table::SymbolTable, Environment},
    error::AlthreadResult,
};

use super::Block;

pub fn consume_while<'a>(
    block: &mut Block,
    symbol_table: &mut SymbolTable,
    env: &mut Environment,
) -> AlthreadResult<bool> {
    if block.current == 0 {
        block.reset();
        if consume_condition(&block.children[0], symbol_table, env)? {
            block.current = 1;
        } else {
            return Ok(false);
        };
    } else if block.current < block.children.len() {
        if !block.children[block.current].consume(symbol_table, env)? {
            block.current = 0;
            return Ok(true);
        }
    }

    Ok(true)
}

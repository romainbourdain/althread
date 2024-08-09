use pest::iterators::Pair;

use crate::{
    env::{symbol_table::SymbolTable, Environment},
    error::AlthreadResult,
    parser::Rule,
};

use super::expr::consume_expr;

pub fn consume_call(
    pair: Pair<Rule>,
    symbol_table: &SymbolTable,
    env: &Environment,
) -> AlthreadResult<()> {
    let value = consume_expr(pair.into_inner().next().unwrap(), symbol_table, env)?;
    println!("{}", value);
    Ok(())
}

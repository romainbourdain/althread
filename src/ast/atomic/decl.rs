use pest::iterators::Pair;

use crate::{
    env::{datatype::DataType, symbol_table::SymbolTable, Environment},
    error::AlthreadResult,
    no_rule,
    parser::Rule,
};

use super::expr::consume_expr;

pub fn consume_decl(
    pair: Pair<Rule>,
    symbol_table: &mut SymbolTable,
    env: &mut Environment,
) -> AlthreadResult<()> {
    let mut pairs = pair.into_inner();
    let mutable = pairs.next().unwrap().as_str() == "let";
    let identifier: Pair<Rule> = pairs.next().unwrap();
    let mut datatype = None;
    let mut value = None;
    for pair in pairs {
        match pair.as_rule() {
            Rule::DATATYPE => datatype = Some(DataType::from_str(pair.as_str())),
            Rule::expr => value = Some(consume_expr(pair, symbol_table, env)?),
            _ => return Err(no_rule!(pair)),
        }
    }

    symbol_table.insert(env, mutable, &identifier, datatype, value)
}

use pest::iterators::Pair;

use crate::{
    env::{symbol_table::SymbolTable, Environment},
    error::{AlthreadError, AlthreadResult, ErrorType},
    no_rule,
    parser::Rule,
};

use super::expr::consume_expr;

pub fn consume_assign(
    pair: Pair<Rule>,
    symbol_table: &mut SymbolTable,
    env: &mut Environment,
) -> AlthreadResult<()> {
    let pair = pair.into_inner().next().unwrap();
    match pair.as_rule() {
        Rule::assign_unary => consume_assign_unary(pair, symbol_table, env),
        Rule::assign_binary => consume_assign_binary(pair, symbol_table, env),
        _ => Err(no_rule!(pair)),
    }
}

fn consume_assign_unary(
    pair: Pair<Rule>,
    symbol_table: &mut SymbolTable,
    env: &mut Environment,
) -> AlthreadResult<()> {
    let mut pairs = pair.into_inner();
    let identifier = pairs.next().unwrap();
    let op = pairs.next().unwrap();

    let current_value = symbol_table.get(env, &identifier)?.value.clone();

    let value = match op.as_str() {
        "++" => current_value.increment(),
        "--" => current_value.decrement(),
        _ => return Err(no_rule!(op)),
    }
    .map_err(|e| {
        AlthreadError::new(
            ErrorType::ArithmeticError,
            identifier.line_col().0,
            identifier.line_col().1,
            e,
        )
    })?;

    symbol_table.update(env, &identifier, value)
}

fn consume_assign_binary(
    pair: Pair<Rule>,
    symbol_table: &mut SymbolTable,
    env: &mut Environment,
) -> AlthreadResult<()> {
    let mut pairs = pair.into_inner();
    let identifier = pairs.next().unwrap();
    let op = pairs.next().unwrap();
    let expr = consume_expr(pairs.next().unwrap(), symbol_table, env)?;

    let current_value = symbol_table.get(env, &identifier)?.value.clone();

    let value = match op.as_str() {
        "=" => Ok(expr),
        "+=" => current_value.add(&expr),
        "-=" => current_value.sub(&expr),
        "*=" => current_value.mul(&expr),
        "/=" => current_value.div(&expr),
        "%=" => current_value.rem(&expr),
        _ => return Err(no_rule!(op)),
    }
    .map_err(|e| {
        AlthreadError::new(
            ErrorType::ArithmeticError,
            identifier.line_col().0,
            identifier.line_col().1,
            e,
        )
    })?;

    symbol_table.update(env, &identifier, value)
}

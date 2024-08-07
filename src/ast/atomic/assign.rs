use pest::iterators::Pair;

use crate::{
    env::Environment,
    error::{AlthreadError, AlthreadResult, ErrorType},
    no_rule,
    parser::Rule,
};

use super::expr::consume_expr;

pub fn consume_assign(pair: Pair<Rule>, env: &mut Environment) -> AlthreadResult<()> {
    let pair = pair.into_inner().next().unwrap();
    match pair.as_rule() {
        Rule::assign_unary => consume_assign_unary(pair, env),
        Rule::assign_binary => consume_assign_binary(pair, env),
        _ => Err(no_rule!(pair)),
    }
}

fn consume_assign_unary(pair: Pair<Rule>, env: &mut Environment) -> AlthreadResult<()> {
    let mut pairs = pair.into_inner();
    let identifier = pairs.next().unwrap();
    let op = pairs.next().unwrap();

    let current_value = env.get_symbol(&identifier)?.value.clone();

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

    env.update_symbol(&identifier, value)
}

fn consume_assign_binary(pair: Pair<Rule>, env: &mut Environment) -> AlthreadResult<()> {
    let mut pairs = pair.into_inner();
    let identifier = pairs.next().unwrap();
    let op = pairs.next().unwrap();
    let expr = consume_expr(pairs.next().unwrap(), env)?;

    let current_value = env.get_symbol(&identifier)?.value.clone();

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

    env.update_symbol(&identifier, value)
}

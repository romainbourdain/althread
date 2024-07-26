use pest::iterators::Pair;

use crate::{
    env::Environment,
    error::{AlthreadError, AlthreadResult, ErrorType},
    no_rule,
    parser::Rule,
};

use super::expr::eval_expr;

pub fn eval_assign(pair: Pair<Rule>, env: &mut Environment) -> AlthreadResult<()> {
    let pair = pair.into_inner().next().unwrap();
    match pair.as_rule() {
        Rule::assign_unary => eval_assign_unary(pair, env),
        Rule::assign_binary => eval_assign_binary(pair, env),
        _ => Err(no_rule!(pair)),
    }
}

fn eval_assign_unary(pair: Pair<Rule>, env: &mut Environment) -> AlthreadResult<()> {
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

fn eval_assign_binary(pair: Pair<Rule>, env: &mut Environment) -> AlthreadResult<()> {
    let mut pairs = pair.into_inner();
    let identifier = pairs.next().unwrap();
    let op = pairs.next().unwrap();
    let expr = eval_expr(pairs.next().unwrap(), env)?;

    let current_value = env.get_symbol(&identifier)?.value.clone();

    let value = match op.as_str() {
        "=" => Ok(current_value),
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

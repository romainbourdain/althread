use std::iter::Rev;

use pest::iterators::{Pair, Pairs};

use crate::{
    env::{datatype::DataType, Environment},
    error::{AlthreadError, AlthreadResult, ErrorType},
    no_rule,
    parser::Rule,
};

pub fn check_expr<'a>(pair: Pair<Rule>, env: &Environment) -> AlthreadResult<DataType> {
    match pair.as_rule() {
        Rule::primary => Ok(check_primary(pair.into_inner().next().unwrap(), env)?),
        Rule::unary => Ok(check_unary(pair.into_inner(), env)?),
        Rule::expr
        | Rule::logical_or
        | Rule::logical_and
        | Rule::equality
        | Rule::comparison
        | Rule::term
        | Rule::factor => Ok(check_binary(pair.into_inner().rev(), env)?),
        _ => {
            return Err(no_rule!(pair));
        }
    }
}

fn check_primary(pair: Pair<Rule>, env: &Environment) -> AlthreadResult<DataType> {
    Ok(match pair.as_rule() {
        Rule::NULL => DataType::Void,
        Rule::BOOLEAN => DataType::Bool,
        Rule::INTEGER => DataType::Int,
        Rule::FLOAT => DataType::Float,
        Rule::STRING => DataType::String,
        Rule::IDENTIFIER => {
            let symbol = env.get_symbol(&pair)?;
            symbol.datatype.clone()
        }
        Rule::expr => check_expr(pair, env)?,
        _ => return Err(no_rule!(pair)),
    })
}

fn check_unary<'a>(mut pairs: Pairs<'a, Rule>, env: &Environment) -> AlthreadResult<DataType> {
    let pair: Pair<'a, Rule> = pairs.next().unwrap();
    if let Some(val) = pairs.next() {
        let val = check_expr(val, env)?;
        let op = pair;
        match op.as_str() {
            "+" | "-" => val.can_neg(),
            "!" => val.can_not(),
            _ => return Err(no_rule!(op)),
        }
        .map_err(|e| {
            AlthreadError::new(
                ErrorType::ArithmeticError,
                op.line_col().0,
                op.line_col().1,
                e.to_string(),
            )
        })?;
        Ok(val)
    } else {
        Ok(check_expr(pair, env)?)
    }
}

fn check_binary<'a>(
    mut pairs: Rev<Pairs<'a, Rule>>,
    env: &Environment,
) -> AlthreadResult<DataType> {
    let right_value = check_expr(pairs.next().unwrap(), env)?;
    if let Some(op) = pairs.next() {
        let left_value = check_binary(pairs, env)?;

        match op.as_str() {
            "+" => left_value.can_add(&right_value),
            "-" | "*" | "/" | "%" => left_value.can_arithmetic(&right_value),
            "==" | "!=" => left_value.can_compare(&right_value),
            "<" | "<=" | ">" | ">=" => left_value.can_order(&right_value),
            "&&" | "||" => left_value.can_logical(&right_value),
            _ => return Err(no_rule!(op)),
        }
        .map_err(|e| {
            AlthreadError::new(
                ErrorType::ArithmeticError,
                op.line_col().0,
                op.line_col().1,
                e.to_string(),
            )
        })?;
        Ok(left_value)
    } else {
        Ok(right_value)
    }
}

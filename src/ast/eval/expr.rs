use std::iter::Rev;

use pest::iterators::{Pair, Pairs};

use crate::{
    env::{value::Value, Environment},
    error::{AlthreadError, AlthreadResult, ErrorType},
    no_rule,
    parser::Rule,
};

pub fn eval_expr<'a>(pair: Pair<Rule>, env: &mut Environment) -> AlthreadResult<Value> {
    match pair.as_rule() {
        Rule::primary => Ok(eval_primary(pair.into_inner().next().unwrap(), env)?),
        Rule::unary => Ok(eval_unary(pair.into_inner(), env)?),
        Rule::expr
        | Rule::logical_or
        | Rule::logical_and
        | Rule::equality
        | Rule::comparison
        | Rule::term
        | Rule::factor => Ok(eval_binary(pair.into_inner().rev(), env)?),
        _ => {
            return Err(no_rule!(pair));
        }
    }
}

fn eval_primary(pair: Pair<Rule>, env: &mut Environment) -> AlthreadResult<Value> {
    let val = pair.as_str();

    Ok(match pair.as_rule() {
        Rule::NULL => Value::Null,
        Rule::BOOLEAN => Value::Bool(val == "true"),
        Rule::INTEGER => Value::Int(val.parse::<i64>().unwrap()),
        Rule::FLOAT => Value::Float(val.parse::<f64>().unwrap()),
        Rule::STRING => Value::String(val.to_string()),
        Rule::IDENTIFIER => {
            let symbol = env.get_symbol(&pair)?;
            symbol.value.clone()
        }
        Rule::expr => eval_expr(pair, env)?,
        _ => return Err(no_rule!(pair)),
    })
}

fn eval_unary<'a>(mut pairs: Pairs<'a, Rule>, env: &mut Environment) -> AlthreadResult<Value> {
    let pair = pairs.next().unwrap();
    match pair.as_rule() {
        Rule::unary_op => {
            let val = eval_unary(pairs, env)?;
            Ok(match pair.as_str() {
                "+" => Ok(val),
                "-" => val.neg(),
                "!" => val.not(),
                _ => return Err(no_rule!(pair)),
            }
            .map_err(|e| {
                AlthreadError::new(
                    ErrorType::ArithmeticError,
                    pair.line_col().0,
                    pair.line_col().1,
                    e.to_string(),
                )
            })?)
        }
        Rule::primary => Ok(eval_expr(pair, env)?),
        _ => Err(no_rule!(pair)),
    }
}

fn eval_binary<'a>(
    mut pairs: Rev<Pairs<'a, Rule>>,
    env: &mut Environment,
) -> AlthreadResult<Value> {
    let right_value = eval_expr(pairs.next().unwrap(), env)?;
    if let Some(op) = pairs.next() {
        let left_value = eval_binary(pairs, env)?;

        Ok(match op.as_str() {
            "+" => left_value.add(&right_value),
            "-" => left_value.sub(&right_value),
            "*" => left_value.mul(&right_value),
            "/" => left_value.div(&right_value),
            "%" => left_value.rem(&right_value),
            "==" => left_value.eq(&right_value),
            "!=" => left_value.ne(&right_value),
            "<" => left_value.lt(&right_value),
            "<=" => left_value.le(&right_value),
            ">" => left_value.gt(&right_value),
            ">=" => left_value.ge(&right_value),
            "&&" => left_value.and(&right_value),
            "||" => left_value.or(&right_value),
            _ => return Err(no_rule!(op)),
        }
        .map_err(|e| {
            AlthreadError::new(
                ErrorType::ArithmeticError,
                op.line_col().0,
                op.line_col().1,
                e.to_string(),
            )
        })?)
    } else {
        Ok(right_value)
    }
}

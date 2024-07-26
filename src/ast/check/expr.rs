use pest::iterators::{Pair, Pairs};

use crate::{
    env::{datatype::DataType, Environment},
    error::{AlthreadError, AlthreadResult, ErrorType},
    no_rule,
    parser::Rule,
};

pub fn check_expr<'a>(pair: Pair<'a, Rule>, env: &mut Environment) -> AlthreadResult<DataType> {
    match pair.as_rule() {
        Rule::primary => Ok(check_primary(pair.into_inner().next().unwrap(), env)?),
        Rule::unary => Ok(check_unary(pair.into_inner(), env)?),
        Rule::expr
        | Rule::logical_or
        | Rule::logical_and
        | Rule::equality
        | Rule::comparison
        | Rule::term
        | Rule::factor => Ok(check_binary(pair.into_inner(), env)?),
        _ => {
            return Err(no_rule!(pair));
        }
    }
}

fn check_primary(pair: Pair<Rule>, env: &mut Environment) -> AlthreadResult<DataType> {
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

fn check_unary<'a>(mut pairs: Pairs<'a, Rule>, env: &mut Environment) -> AlthreadResult<DataType> {
    let pair = pairs.next().unwrap();
    match pair.as_rule() {
        Rule::unary_op => {
            let data_type = check_unary(pairs, env)?;

            match pair.as_str() {
                "+" if data_type.is_numeric() => Ok(data_type),
                "-" if data_type.is_numeric() => Ok(data_type),
                "!" if data_type == DataType::Bool => Ok(data_type),
                op => Err(format!(
                    "Wrong type for {} unary operator: {}",
                    op, data_type
                )),
            }
            .map_err(|e| {
                AlthreadError::new(
                    ErrorType::TypeError,
                    pair.line_col().0,
                    pair.line_col().1,
                    e,
                )
            })
        }
        Rule::primary => Ok(check_expr(pair, env)?),
        _ => Err(no_rule!(pair)),
    }
}

fn check_binary<'a>(mut pairs: Pairs<'a, Rule>, env: &mut Environment) -> AlthreadResult<DataType> {
    let left_type = check_expr(pairs.next().unwrap(), env)?;
    if let Some(op) = pairs.next() {
        let (line, col) = op.line_col();
        let op = op.as_str();
        let right_type = check_binary(pairs, env)?;
        match op {
            _ if right_type != left_type => Err(format!(
                "{} operation between {} and {} is not allowed",
                op, left_type, right_type
            )),
            "+" | "-" | "*" | "/" | "%" if !left_type.is_numeric() => {
                Err(format!("Wrong type for {} operator: {}", op, left_type))
            }
            "<" | ">" | "<=" | ">=" if !left_type.is_numeric() => {
                Err(format!("Wrong type for {} operator: {}", op, left_type))
            }
            "&&" | "||" if left_type != DataType::Bool => {
                Err(format!("Wrong type for {} operator: {}", op, left_type))
            }
            "==" | "!=" | "<" | ">" | "<=" | ">=" => Ok(DataType::Bool),
            _ => Ok(left_type),
        }
        .map_err(|e| AlthreadError::new(ErrorType::TypeError, line, col, e))
    } else {
        Ok(left_type)
    }
}

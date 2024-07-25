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
            let symbol = env.get_symbol(pair)?;
            symbol.datatype.clone()
        }
        _ => return Err(no_rule!(pair)),
    })
}

fn check_unary<'a>(mut pairs: Pairs<'a, Rule>, env: &mut Environment) -> AlthreadResult<DataType> {
    let pair = pairs.next().unwrap();
    match pair.as_rule() {
        Rule::unary_op => {
            let data_type = check_unary(pairs, env)?;
            let error_message = |op: &str, data_type: DataType| {
                AlthreadError::new(
                    ErrorType::TypeError,
                    pair.line_col().0,
                    pair.line_col().1,
                    format!("Wrong type for {} unary operator: {}", op, data_type),
                )
            };

            match pair.as_str() {
                "-" if !data_type.is_numeric() => Err(error_message(pair.as_str(), data_type)),
                "!" if data_type != DataType::Bool => Err(error_message(pair.as_str(), data_type)),
                _ => Ok(data_type),
            }
        }
        Rule::primary => Ok(check_expr(pair, env)?),
        _ => Err(no_rule!(pair)),
    }
}

fn check_binary<'a>(mut pairs: Pairs<'a, Rule>, env: &mut Environment) -> AlthreadResult<DataType> {
    let error_message = |op: Pair<Rule>, data_type: DataType| {
        AlthreadError::new(
            ErrorType::TypeError,
            op.line_col().0,
            op.line_col().1,
            format!("Wrong type for {} operator: {}", op.as_str(), data_type),
        )
    };

    let left_type = check_expr(pairs.next().unwrap(), env)?;
    if let Some(op) = pairs.next() {
        let right_type = check_binary(pairs, env)?;
        match op.as_str() {
            "+" | "-" | "*" | "/" | "%" if !left_type.is_numeric() => {
                Err(error_message(op, left_type))
            }
            "<" | ">" | "<=" | ">=" if !left_type.is_numeric() => Err(error_message(op, left_type)),
            "&&" | "||" if left_type != DataType::Bool => Err(error_message(op, left_type)),
            "==" | "!=" | "<" | ">" | "<=" | ">=" => Ok(DataType::Bool),
            _ if right_type != left_type => Err(AlthreadError::new(
                ErrorType::TypeError,
                op.line_col().0,
                op.line_col().1,
                format!(
                    "{} operation between {} and {} is not allowed",
                    op.as_str(),
                    left_type,
                    right_type
                ),
            )),
            _ => Ok(left_type),
        }
    } else {
        Ok(left_type)
    }
}

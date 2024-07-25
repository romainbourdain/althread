use std::iter::Rev;

use pest::iterators::{Pair, Pairs};

use crate::{
    env::{
        datatype::{self, DataType},
        value::Value,
        Environment,
    },
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
            symbol.value.clone().unwrap()
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
            let error_message = |op: &str, data_type: DataType| {
                AlthreadError::new(
                    ErrorType::TypeError,
                    pair.line_col().0,
                    pair.line_col().1,
                    format!("Wrong type for {} unary operator: {}", op, data_type),
                )
            };

            match pair.as_str() {
                "+" => Ok(val),
                "-" => match val {
                    Value::Int(val) => Ok(Value::Int(-val)),
                    Value::Float(val) => Ok(Value::Float(-val)),
                    _ => Err(error_message(pair.as_str(), val.get_type())),
                },
                "!" => match val {
                    Value::Bool(val) => Ok(Value::Bool(!val)),
                    _ => Err(error_message(pair.as_str(), val.get_type())),
                },
                _ => Err(no_rule!(pair)),
            }
        }
        Rule::primary => Ok(eval_expr(pair, env)?),
        _ => Err(no_rule!(pair)),
    }
}

fn eval_binary<'a>(
    mut pairs: Rev<Pairs<'a, Rule>>,
    env: &mut Environment,
) -> AlthreadResult<Value> {
    let error_message = |op: Pair<Rule>, data_type: DataType| {
        AlthreadError::new(
            ErrorType::TypeError,
            op.line_col().0,
            op.line_col().1,
            format!("Wrong type for {} operator: {}", op.as_str(), data_type),
        )
    };

    let right_value = eval_expr(pairs.next().unwrap(), env)?;
    if let Some(op) = pairs.next() {
        let left_value = eval_binary(pairs, env)?;

        let datatype = left_value.get_type();
        let is_right_null = right_value.as_str() == "0";

        match op.as_str() {
            "+" => match (left_value, right_value) {
                (Value::Int(left), Value::Int(right)) => Ok(Value::Int(left + right)),
                (Value::Float(left), Value::Float(right)) => Ok(Value::Float(left + right)),
                (Value::String(left), Value::String(right)) => {
                    Ok(Value::String(format!("{}{}", left, right)))
                }
                _ => Err(error_message(op, datatype)),
            },
            "-" => match (left_value, right_value) {
                (Value::Int(left), Value::Int(right)) => Ok(Value::Int(left - right)),
                (Value::Float(left), Value::Float(right)) => Ok(Value::Float(left - right)),
                _ => Err(error_message(op, datatype)),
            },
            "*" => match (left_value, right_value) {
                (Value::Int(left), Value::Int(right)) => Ok(Value::Int(left * right)),
                (Value::Float(left), Value::Float(right)) => Ok(Value::Float(left * right)),
                _ => Err(error_message(op, datatype)),
            },
            "/" => match (left_value, right_value) {
                _ if is_right_null => {
                    return Err(AlthreadError::new(
                        ErrorType::DivisionByZero,
                        op.line_col().0,
                        op.line_col().1,
                        "Division by zero".to_string(),
                    ));
                }
                (Value::Int(left), Value::Int(right)) => Ok(Value::Int(left / right)),
                (Value::Float(left), Value::Float(right)) => Ok(Value::Float(left / right)),
                _ => Err(error_message(op, datatype)),
            },
            "%" => match (left_value, right_value) {
                _ if is_right_null => {
                    return Err(AlthreadError::new(
                        ErrorType::DivisionByZero,
                        op.line_col().0,
                        op.line_col().1,
                        "Division by zero".to_string(),
                    ));
                }
                (Value::Int(left), Value::Int(right)) => Ok(Value::Int(left % right)),
                (Value::Float(left), Value::Float(right)) => Ok(Value::Float(left % right)),
                _ => Err(error_message(op, datatype)),
            },
            "==" => Ok(Value::Bool(left_value.as_str() == right_value.as_str())),
            "!=" => Ok(Value::Bool(left_value.as_str() != right_value.as_str())),
            "<" => match (left_value, right_value) {
                (Value::Int(left), Value::Int(right)) => Ok(Value::Bool(left < right)),
                (Value::Float(left), Value::Float(right)) => Ok(Value::Bool(left < right)),
                _ => Err(error_message(op, datatype)),
            },
            "<=" => match (left_value, right_value) {
                (Value::Int(left), Value::Int(right)) => Ok(Value::Bool(left <= right)),
                (Value::Float(left), Value::Float(right)) => Ok(Value::Bool(left <= right)),
                _ => Err(error_message(op, datatype)),
            },
            ">" => match (left_value, right_value) {
                (Value::Int(left), Value::Int(right)) => Ok(Value::Bool(left > right)),
                (Value::Float(left), Value::Float(right)) => Ok(Value::Bool(left > right)),
                _ => Err(error_message(op, datatype)),
            },
            ">=" => match (left_value, right_value) {
                (Value::Int(left), Value::Int(right)) => Ok(Value::Bool(left >= right)),
                (Value::Float(left), Value::Float(right)) => Ok(Value::Bool(left >= right)),
                _ => Err(error_message(op, datatype)),
            },
            "&&" => match (left_value, right_value) {
                (Value::Bool(left), Value::Bool(right)) => Ok(Value::Bool(left && right)),
                _ => Err(error_message(op, datatype)),
            },
            "||" => match (left_value, right_value) {
                (Value::Bool(left), Value::Bool(right)) => Ok(Value::Bool(left || right)),
                _ => Err(error_message(op, datatype)),
            },
            _ => Err(no_rule!(op)),
        }
    } else {
        Ok(right_value)
    }
}

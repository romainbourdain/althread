use pest::iterators::Pair;

use crate::{
    env::Environment,
    error::{AlthreadError, AlthreadResult, ErrorType},
    no_rule,
    parser::Rule,
};

use super::expr::check_expr;

pub fn check_assign(pair: Pair<Rule>, env: &mut Environment) -> AlthreadResult<()> {
    let pair = pair.into_inner().next().unwrap();
    match pair.as_rule() {
        Rule::assign_unary => check_assign_unary(pair, env),
        Rule::assign_binary => check_assign_binary(pair, env),
        _ => Err(no_rule!(pair)),
    }
}

fn check_assign_unary(pair: Pair<Rule>, env: &mut Environment) -> AlthreadResult<()> {
    let mut pairs = pair.into_inner();
    let identifier = pairs.next().unwrap();
    let op = pairs.next().unwrap();

    let ident_type = env.get_symbol(&identifier)?.datatype.clone();

    match op.as_str() {
        "++" | "--" => ident_type.can_unary(),
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

    Ok(())
}

fn check_assign_binary(pair: Pair<Rule>, env: &mut Environment) -> AlthreadResult<()> {
    let mut pairs = pair.into_inner();
    let identifier = pairs.next().unwrap();
    let op = pairs.next().unwrap();
    let expr_type = check_expr(pairs.next().unwrap(), env)?;

    let ident_type = env.get_symbol(&identifier)?.datatype.clone();

    match op.as_str() {
        "=" => {
            if expr_type == ident_type {
                Ok(())
            } else {
                return Err(AlthreadError::new(
                    ErrorType::TypeError,
                    identifier.line_col().0,
                    identifier.line_col().1,
                    format!(
                        "Expected type {:?}, found {:?}",
                        ident_type.to_string(),
                        expr_type.to_string()
                    ),
                ));
            }
        }
        "+=" => ident_type.can_add(&expr_type),
        "-=" | "*=" | "/=" | "%=" => ident_type.can_arithmetic(&expr_type),
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

    Ok(())
}

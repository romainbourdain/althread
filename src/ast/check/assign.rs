use pest::iterators::Pair;

use crate::{
    env::Environment,
    error::{AlthreadError, AlthreadResult, ErrorType},
    no_rule,
    parser::Rule,
};

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
    let symbol = env.get_symbol(&identifier)?;

    if !symbol.mutable {
        return Err(AlthreadError::new(
            ErrorType::TypeError,
            identifier.line_col().0,
            identifier.line_col().1,
            format!(
                "Cannot assign to immutable variable '{}'",
                identifier.as_str()
            ),
        ));
    }

    match op.as_str() {
        "++" | "--" if !symbol.datatype.is_numeric() => Err(AlthreadError::new(
            ErrorType::TypeError,
            op.line_col().0,
            op.line_col().1,
            format!("Cannot use '{}' on {} type", op.as_str(), symbol.datatype),
        )),
        _ => Ok(()),
    }
}

fn check_assign_binary(pair: Pair<Rule>, env: &mut Environment) -> AlthreadResult<()> {
    Ok(())
}

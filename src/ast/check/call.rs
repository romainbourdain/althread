use pest::iterators::Pair;

use crate::{env::Environment, error::AlthreadResult, parser::Rule};

use super::expr::check_expr;

pub fn check_call(pair: Pair<Rule>, env: &mut Environment) -> AlthreadResult<()> {
    check_expr(pair.into_inner().next().unwrap(), env)?;
    Ok(())
}

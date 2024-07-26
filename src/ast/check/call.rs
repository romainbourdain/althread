use pest::iterators::Pair;

use crate::{ast::eval::expr::eval_expr, env::Environment, error::AlthreadResult, parser::Rule};

pub fn check_call(pair: Pair<Rule>, env: &mut Environment) -> AlthreadResult<()> {
    eval_expr(pair.into_inner().next().unwrap(), env)?;
    Ok(())
}

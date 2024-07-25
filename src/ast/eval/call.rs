use pest::iterators::Pair;

use crate::{env::Environment, error::AlthreadResult, parser::Rule};

use super::expr::eval_expr;

pub fn eval_call(pair: Pair<Rule>, env: &mut Environment) -> AlthreadResult<()> {
    let value = eval_expr(pair.into_inner().next().unwrap(), env)?;
    println!("{}", value);
    Ok(())
}

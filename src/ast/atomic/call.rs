use pest::iterators::Pair;

use crate::{env::Environment, error::AlthreadResult, parser::Rule};

use super::expr::consume_expr;

pub fn consume_call(pair: Pair<Rule>, env: &mut Environment) -> AlthreadResult<()> {
    let value = consume_expr(pair.into_inner().next().unwrap(), env)?;
    println!("{}", value);
    Ok(())
}

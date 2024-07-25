use pest::iterators::Pair;

use crate::{error::AlthreadResult, parser::Rule};

use super::expr::check_expr;

pub fn check_call(pair: Pair<Rule>) -> AlthreadResult<()> {
    check_expr(pair.into_inner().next().unwrap())?;
    Ok(())
}

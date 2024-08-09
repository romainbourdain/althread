use pest::iterators::Pair;

use crate::{error::AlthreadResult, parser::Rule};

use super::expr::Expr;

#[derive(Debug)]
pub struct Print {
    pub value: Expr,
    pub line: usize,
    pub column: usize,
}

impl Print {
    pub fn build(pair: Pair<Rule>) -> AlthreadResult<Self> {
        let (line, column) = pair.line_col();
        let mut pairs = pair.into_inner();

        let value = Expr::build(pairs.next().unwrap())?;

        Ok(Self {
            value,
            line,
            column,
        })
    }
}

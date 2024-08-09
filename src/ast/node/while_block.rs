use pest::iterators::Pair;

use crate::{error::AlthreadResult, parser::Rule};

use super::{expr::Expr, scope::Scope};

#[derive(Debug)]
pub struct WhileBlock {
    pub condition: Expr,
    pub then_block: Scope,
    pub line: usize,
    pub column: usize,
}

impl WhileBlock {
    pub fn build(pair: Pair<Rule>) -> AlthreadResult<Self> {
        let (line, column) = pair.line_col();
        let mut pairs = pair.into_inner();

        let condition = Expr::build(pairs.next().unwrap())?;
        let then_block = Scope::build(pairs.next().unwrap())?;

        Ok(Self {
            condition,
            then_block,
            line,
            column,
        })
    }
}

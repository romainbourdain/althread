use pest::iterators::Pair;

use crate::{error::AlthreadResult, parser::Rule};

use super::{expr::Expr, scope::Scope};

#[derive(Debug)]
pub struct IfBlock {
    pub condition: Expr,
    pub then_block: Scope,
    pub else_block: Option<Scope>,
    pub line: usize,
    pub column: usize,
}

impl IfBlock {
    pub fn build(pair: Pair<Rule>) -> AlthreadResult<Self> {
        let (line, column) = pair.line_col();
        let mut pairs = pair.into_inner();

        let condition = Expr::build(pairs.next().unwrap())?;
        let then_block = Scope::build(pairs.next().unwrap())?;
        let else_block = pairs.next().map(|pair| Scope::build(pair)).transpose()?;

        Ok(Self {
            condition,
            then_block,
            else_block,
            line,
            column,
        })
    }
}

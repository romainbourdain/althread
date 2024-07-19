use pest::iterators::Pair;

use crate::{ast::expr::Expr, env::Environment, error::AlthreadError, parser::Rule};

#[derive(Debug)]
pub struct PrintStmt {
    pub expr: Expr,
    pub line: usize,
    pub column: usize,
}

impl PrintStmt {
    pub fn build(pair: Pair<Rule>, env: &Environment) -> Result<Self, AlthreadError> {
        let (line, column) = pair.line_col();
        let expr = Expr::build(pair.into_inner().next().unwrap(), env)?;
        Ok(Self { expr, line, column })
    }
}

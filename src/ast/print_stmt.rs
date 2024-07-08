use pest::iterators::Pair;

use crate::{env::Environment, error::AlthreadError, parser::Rule};

use super::expr::Expr;

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

    pub fn eval(&self) -> Result<(), AlthreadError> {
        let value = self.expr.eval()?;
        println!("{}", value);
        Ok(())
    }
}

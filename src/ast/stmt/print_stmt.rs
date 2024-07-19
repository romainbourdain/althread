use std::io::Write;

use pest::iterators::Pair;

use crate::{
    ast::expr::Expr,
    env::Environment,
    error::{AlthreadError, ErrorType},
    parser::Rule,
};

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

    pub fn eval<W>(&self, env: &mut Environment, output: &mut W) -> Result<(), AlthreadError>
    where
        W: Write,
    {
        let value = self.expr.eval(env)?;
        write!(output, "{}\n", value).map_err(|e| {
            AlthreadError::error(
                ErrorType::RuntimeError,
                self.line,
                self.column,
                e.to_string(),
            )
        })?;

        Ok(())
    }
}

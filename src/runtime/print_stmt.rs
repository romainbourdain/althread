use std::io::Write;

use crate::{
    ast::stmt::print_stmt::PrintStmt,
    env::Environment,
    error::{AlthreadError, ErrorType},
};

impl PrintStmt {
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

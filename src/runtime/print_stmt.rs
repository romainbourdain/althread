use std::io::Write;

use crate::{
    env::Environment,
    error::{AlthreadError, ErrorType},
    nodes::print_stmt::PrintStmt,
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

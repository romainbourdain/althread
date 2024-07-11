use std::io::Write;

use crate::{
    env::Environment,
    error::{AlthreadError, ErrorType},
    nodes::{expr::primary::PrimaryExpr, while_stmt::WhileStmt},
};

impl WhileStmt {
    pub fn eval<W>(&self, env: &mut Environment, output: &mut W) -> Result<(), AlthreadError>
    where
        W: Write,
    {
        loop {
            match self.condition.eval(env)? {
                PrimaryExpr::Bool(true) => self.block.eval(env, output)?,
                PrimaryExpr::Bool(false) => break,
                _ => {
                    return Err(AlthreadError::error(
                        ErrorType::RuntimeError,
                        self.line,
                        self.column,
                        format!("Condition must be a boolean"),
                    ))
                }
            }
        }

        Ok(())
    }
}

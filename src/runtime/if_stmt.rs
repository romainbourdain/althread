use std::io::Write;

use crate::{
    ast::{expr::primary::PrimaryExpr, stmt::if_stmt::IfStmt},
    env::Environment,
    error::{AlthreadError, ErrorType},
};

impl IfStmt {
    pub fn eval<W>(&self, env: &mut Environment, output: &mut W) -> Result<(), AlthreadError>
    where
        W: Write,
    {
        match self.condition.eval(env)? {
            PrimaryExpr::Bool(true) => self.block.eval(env, output)?,
            PrimaryExpr::Bool(false) => {
                if let Some(else_block) = &self.else_block {
                    else_block.eval(env, output)?;
                }
            }
            _ => {
                return Err(AlthreadError::error(
                    ErrorType::RuntimeError,
                    self.line,
                    self.column,
                    format!("Condition must be a boolean"),
                ))
            }
        }

        Ok(())
    }
}

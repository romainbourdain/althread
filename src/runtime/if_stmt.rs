use crate::{
    env::Environment,
    error::{AlthreadError, ErrorType},
    nodes::{expr::primary::PrimaryExpr, if_stmt::IfStmt},
};

impl IfStmt {
    pub fn eval(&self, env: &mut Environment) -> Result<(), AlthreadError> {
        match self.condition.eval(env)? {
            PrimaryExpr::Bool(true) => self.block.eval(env)?,
            PrimaryExpr::Bool(false) => {
                if let Some(else_block) = &self.else_block {
                    else_block.eval(env)?;
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

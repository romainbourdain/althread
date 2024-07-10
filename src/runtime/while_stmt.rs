use crate::{
    env::Environment,
    error::{AlthreadError, ErrorType},
    nodes::{expr::primary::PrimaryExpr, while_stmt::WhileStmt},
};

impl WhileStmt {
    pub fn eval(&self, env: &mut Environment) -> Result<(), AlthreadError> {
        loop {
            match self.condition.eval(env)? {
                PrimaryExpr::Bool(true) => self.block.eval(env)?,
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

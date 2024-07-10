use crate::{env::Environment, error::AlthreadError, nodes::while_stmt::WhileStmt};

impl WhileStmt {
    pub fn eval(&self, env: &mut Environment) -> Result<(), AlthreadError> {
        // TODO: Implement while statement evaluation
        unimplemented!();
    }
}

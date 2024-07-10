use crate::{env::Environment, error::AlthreadError, nodes::if_stmt::IfStmt};

impl IfStmt {
    pub fn eval(&self, env: &mut Environment) -> Result<(), AlthreadError> {
        // TODO: Implement if statement evaluation
        unimplemented!();
    }
}

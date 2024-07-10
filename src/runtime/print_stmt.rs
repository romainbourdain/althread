use crate::{env::Environment, error::AlthreadError, nodes::print_stmt::PrintStmt};

impl PrintStmt {
    pub fn eval(&self, env: &mut Environment) -> Result<(), AlthreadError> {
        let value = self.expr.eval(env)?;
        println!("{}", value);
        Ok(())
    }
}

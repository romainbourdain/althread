use crate::{env::Environment, error::AlthreadError, nodes::block::Block};

impl Block {
    pub fn eval(&self, env: &mut Environment) -> Result<(), AlthreadError> {
        for stmt in &self.stmts {
            stmt.eval(env)?;
        }

        Ok(())
    }
}

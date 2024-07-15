use std::io::Write;

use crate::{env::Environment, error::AlthreadError, nodes::block::Block};

impl Block {
    pub fn eval<W>(&self, env: &mut Environment, output: &mut W) -> Result<(), AlthreadError>
    where
        W: Write,
    {
        for stmt in &self.stmts {
            stmt.eval(env, output)?;
        }

        Ok(())
    }

    pub fn eval_and_push<W>(
        &self,
        env: &mut Environment,
        output: &mut W,
    ) -> Result<(), AlthreadError>
    where
        W: Write,
    {
        env.push_table();
        self.eval(env, output)?;
        env.pop_table();
        Ok(())
    }
}

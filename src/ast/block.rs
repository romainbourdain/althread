use pest::iterators::Pair;

use crate::{
    env::Environment,
    error::AlthreadError,
    nodes::{block::Block, stmt::Stmt},
    parser::Rule,
};

impl Block {
    pub fn parse(pair: Pair<Rule>, env: &mut Environment) -> Result<Self, AlthreadError> {
        let (line, column) = pair.line_col();
        let mut block = Self::new(line, column);

        for pair in pair.into_inner() {
            block.stmts.push(Stmt::build(pair, env)?);
        }

        Ok(block)
    }

    pub fn parse_and_push(pair: Pair<Rule>, env: &mut Environment) -> Result<Self, AlthreadError> {
        env.push_table();
        let block = Self::parse(pair, env)?;
        env.pop_table();
        Ok(block)
    }

    pub fn eval(&self, env: &mut Environment) -> Result<(), AlthreadError> {
        for stmt in &self.stmts {
            stmt.eval(env)?;
        }

        Ok(())
    }
}

use pest::iterators::Pair;

use crate::{
    env::Environment,
    error::AlthreadError,
    nodes::{expr::Expr, print_stmt::PrintStmt},
    parser::Rule,
};

impl PrintStmt {
    pub fn build(pair: Pair<Rule>, env: &Environment) -> Result<Self, AlthreadError> {
        let (line, column) = pair.line_col();
        let expr = Expr::build(pair.into_inner().next().unwrap(), env)?;
        Ok(Self { expr, line, column })
    }

    pub fn eval(&self, env: &mut Environment) -> Result<(), AlthreadError> {
        let value = self.expr.eval(env)?;
        println!("{}", value);
        Ok(())
    }
}

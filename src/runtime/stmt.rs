use std::io::Write;

use crate::{env::Environment, error::AlthreadError, nodes::stmt::Stmt};

impl Stmt {
    pub fn eval<W>(&self, env: &mut Environment, output: &mut W) -> Result<(), AlthreadError>
    where
        W: Write,
    {
        use Stmt::*;
        match self {
            Assign(assign) => assign.eval(env)?,
            Expr(expr) => {
                expr.eval(env)?;
            }
            Decl(decl) => decl.eval(env)?,
            Print(print_stmt) => print_stmt.eval(env, output)?,
            Block(block) => block.eval(env, output)?,
            IfStmt(if_stmt) => if_stmt.eval(env, output)?,
            WhileStmt(while_stmt) => while_stmt.eval(env, output)?,
        };
        Ok(())
    }
}

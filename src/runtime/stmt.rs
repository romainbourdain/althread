use crate::{env::Environment, error::AlthreadError, nodes::stmt::Stmt};

impl Stmt {
    pub fn eval(&self, env: &mut Environment) -> Result<(), AlthreadError> {
        use Stmt::*;
        match self {
            Expr(expr) => {
                expr.eval(env)?;
            }
            Decl(decl) => decl.eval(env)?,
            Print(print_stmt) => print_stmt.eval(env)?,
            Block(block) => block.eval(env)?,
            Assign(assign) => assign.eval(env)?,
            IfStmt(if_stmt) => if_stmt.eval(env)?,
            WhileStmt(while_stmt) => while_stmt.eval(env)?,
        };
        Ok(())
    }
}

use crate::{env::Environment, error::AlthreadError, nodes::Ast};

pub mod assign;
pub mod block;
pub mod datatype;
pub mod decl;
pub mod expr;
pub mod if_stmt;
pub mod print_stmt;
pub mod stmt;
pub mod while_stmt;

impl Ast {
    pub fn eval(&self, env: &mut Environment) -> Result<(), AlthreadError> {
        self.shared_block.as_ref().map(|block| block.eval(env));
        env.push_table();
        self.main_block.as_ref().map(|block| block.eval(env));

        Ok(())
    }
}

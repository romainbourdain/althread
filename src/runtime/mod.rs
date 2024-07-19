use std::io::Write;

use crate::{ast::Ast, env::Environment, error::AlthreadError};

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
    pub fn eval<W>(&self, env: &mut Environment, output: &mut W) -> Result<(), AlthreadError>
    where
        W: Write,
    {
        if let Some(block) = self.shared_block.as_ref() {
            block.eval(env, output)?;
        }
        if let Some(block) = self.main_block.as_ref() {
            block.eval_and_push(env, output)?;
        }

        Ok(())
    }
}

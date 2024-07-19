pub mod binary_assign;
pub mod unary_assign;

use crate::{ast::stmt::assign::Assign, env::Environment, error::AlthreadError};

impl Assign {
    pub fn eval(&self, env: &mut Environment) -> Result<(), AlthreadError> {
        match self {
            Assign::Binary(binary) => binary.eval(env),
            Assign::Unary(unary) => unary.eval(env),
        }
    }
}

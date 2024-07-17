use crate::{
    env::Environment,
    error::AlthreadError,
    ast::expr::{
        primary::PrimaryExpr,
        unary::{UnExpr, UnOp},
    },
};

impl UnExpr {
    pub fn eval(&self, env: &Environment) -> Result<PrimaryExpr, AlthreadError> {
        let rhs = self.rhs.eval(env)?;
        match self.op {
            UnOp::Not => match rhs {
                PrimaryExpr::Bool(v) => Ok(PrimaryExpr::Bool(!v)),
                _ => unreachable!(),
            },
            UnOp::Neg => match rhs {
                PrimaryExpr::Int(v) => Ok(PrimaryExpr::Int(-v)),
                PrimaryExpr::Float(v) => Ok(PrimaryExpr::Float(-v)),
                _ => unreachable!(),
            },
        }
    }
}

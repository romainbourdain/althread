pub mod binary;
pub mod primary;
pub mod unary;

use crate::{
    env::Environment,
    error::AlthreadError,
    nodes::expr::{primary::PrimaryExpr, Expr, ExprKind},
};

impl Expr {
    pub fn eval(&self, env: &Environment) -> Result<PrimaryExpr, AlthreadError> {
        match &self.kind {
            ExprKind::Primary(expr) => expr.eval(env),
            ExprKind::Unary(expr) => expr.eval(env),
            ExprKind::Binary(expr) => expr.eval(env),
        }
    }
}

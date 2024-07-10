use crate::{
    env::Environment,
    error::AlthreadError,
    nodes::expr::{
        binary::{BinExpr, BinOp},
        primary::PrimaryExpr,
    },
};

macro_rules! match_bin {
    ([$(($variant:ident, $out:ident)),*], $lhs:expr, $rhs:expr, $op:expr) => {
        match ($lhs, $rhs) {
            $(
                (PrimaryExpr::$variant(a), PrimaryExpr::$variant(b)) => Ok(PrimaryExpr::$out($op(a, b))),
            )*
            _ => unreachable!(),
        }
    };
}

impl BinExpr {
    pub fn eval(&self, env: &Environment) -> Result<PrimaryExpr, AlthreadError> {
        let lhs = self.lhs.eval(env)?;
        let rhs = self.rhs.eval(env)?;

        match self.op {
            BinOp::Add => match_bin!([(Int, Int), (Float, Float)], lhs, rhs, |a, b| a + b),
            BinOp::Sub => match_bin!([(Int, Int), (Float, Float)], lhs, rhs, |a, b| a - b),
            BinOp::Mul => match_bin!([(Int, Int), (Float, Float)], lhs, rhs, |a, b| a * b),
            BinOp::Div => match_bin!([(Int, Int), (Float, Float)], lhs, rhs, |a, b| a / b),
            BinOp::Mod => match_bin!([(Int, Int), (Float, Float)], lhs, rhs, |a, b| a % b),
            BinOp::Eq => match_bin!([(Int, Bool), (Float, Bool)], lhs, rhs, |a, b| a == b),
            BinOp::Ne => match_bin!([(Int, Bool), (Float, Bool)], lhs, rhs, |a, b| a != b),
            BinOp::Gt => match_bin!([(Int, Bool), (Float, Bool)], lhs, rhs, |a, b| a > b),
            BinOp::Ge => match_bin!([(Int, Bool), (Float, Bool)], lhs, rhs, |a, b| a >= b),
            BinOp::Lt => match_bin!([(Int, Bool), (Float, Bool)], lhs, rhs, |a, b| a < b),
            BinOp::Le => match_bin!([(Int, Bool), (Float, Bool)], lhs, rhs, |a, b| a <= b),
            BinOp::Or => match_bin!([(Bool, Bool)], lhs, rhs, |a, b| a || b),
            BinOp::And => match_bin!([(Bool, Bool)], lhs, rhs, |a, b| a && b),
        }
    }
}
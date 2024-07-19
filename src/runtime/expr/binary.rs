use crate::{
    ast::{
        expr::{binary::BinExpr, primary::PrimaryExpr},
        token::binary_op::BinOp,
    },
    env::Environment,
    error::{AlthreadError, ErrorType},
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
            BinOp::Div => match (&lhs, &rhs) {
                (PrimaryExpr::Int(_), PrimaryExpr::Int(b)) if b == &0 => Err(AlthreadError::error(
                    ErrorType::RuntimeError,
                    self.line,
                    self.column,
                    format!("Division by zero"),
                )),
                (PrimaryExpr::Float(_), PrimaryExpr::Float(b)) if b == &0.0 => {
                    Err(AlthreadError::error(
                        ErrorType::RuntimeError,
                        self.line,
                        self.column,
                        format!("Division by zero"),
                    ))
                }
                _ => match_bin!([(Int, Int), (Float, Float)], lhs, rhs, |a, b| a / b),
            },
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

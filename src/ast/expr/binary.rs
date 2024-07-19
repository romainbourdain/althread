use pest::iterators::Pair;

use crate::{
    ast::token::{binary_op::BinOp, datatype::DataType},
    env::Environment,
    error::{AlthreadError, ErrorType},
    parser::Rule,
};

use super::{primary::PrimaryExpr, Expr, ExprKind, ExprResult};

#[derive(Debug)]
pub struct BinExpr {
    pub lhs: Box<Expr>,
    pub op: BinOp,
    pub rhs: Box<Expr>,
    pub line: usize,
    pub column: usize,
}

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
    pub fn from_pair(
        lhs: ExprResult,
        op: Pair<Rule>,
        rhs: ExprResult,
        env: &Environment,
    ) -> ExprResult {
        let (line, column) = op.line_col();
        let op = BinOp::from_pair(op)?;
        let expr = Self {
            op,
            lhs: Box::new(lhs?),
            rhs: Box::new(rhs?),
            line,
            column,
        };

        expr.get_datatype(env)?;

        Ok(Expr {
            kind: ExprKind::Binary(expr),
            line,
            column,
        })
    }

    pub fn get_datatype(&self, env: &Environment) -> Result<DataType, AlthreadError> {
        let lhs_type = self.lhs.get_datatype(env)?;
        let rhs_type = self.rhs.get_datatype(env)?;
        // TODO : implement error with line and col
        if lhs_type != rhs_type {
            return Err(AlthreadError::error(
                ErrorType::TypeError,
                self.line,
                self.column,
                format!(
                    "Cannot make {} operation between {} and {}",
                    self.op, lhs_type, rhs_type
                ),
            ));
        }

        // TODO : implement with macro
        match self.op {
            // +, -, *, / between (float | int) -> (float | int)
            BinOp::Add | BinOp::Sub | BinOp::Mul | BinOp::Div | BinOp::Mod => {
                if (lhs_type != DataType::Int) && (lhs_type != DataType::Float) {
                    // TODO : implement error with line and col
                    return Err(AlthreadError::error(
                        ErrorType::TypeError,
                        self.line,
                        self.column,
                        format!(
                            "Cannot make {} operation between {} and {}",
                            self.op, lhs_type, rhs_type
                        ),
                    ));
                }
                Ok(lhs_type)
            }

            // ==, !=, <, <=, >, >= between (float | int) -> bool
            BinOp::Eq | BinOp::Ne | BinOp::Gt | BinOp::Ge | BinOp::Lt | BinOp::Le => {
                if (lhs_type != DataType::Int) && (lhs_type != DataType::Float) {
                    // TODO : implement error with line and col
                    return Err(AlthreadError::error(
                        ErrorType::TypeError,
                        self.line,
                        self.column,
                        format!(
                            "Cannot make {} operation between {} and {}",
                            self.op, lhs_type, rhs_type
                        ),
                    ));
                }
                Ok(DataType::Bool)
            }

            // &&, || between bool -> bool
            BinOp::And | BinOp::Or => {
                if lhs_type != DataType::Bool {
                    // TODO : implement error with line and col
                    return Err(AlthreadError::error(
                        ErrorType::TypeError,
                        self.line,
                        self.column,
                        format!(
                            "Cannot make {} operation between {} and {}",
                            self.op, lhs_type, rhs_type
                        ),
                    ));
                }
                Ok(DataType::Bool)
            }
        }
    }

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

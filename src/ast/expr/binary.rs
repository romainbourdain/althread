use pest::iterators::Pair;

use crate::{
    ast::token::{binary_op::BinOp, datatype::DataType},
    env::Environment,
    error::{AlthreadError, ErrorType},
    parser::Rule,
};

use super::{Expr, ExprKind, ExprResult};

#[derive(Debug)]
pub struct BinExpr {
    pub lhs: Box<Expr>,
    pub op: BinOp,
    pub rhs: Box<Expr>,
    pub line: usize,
    pub column: usize,
}

impl BinExpr {
    pub fn build(
        lhs: ExprResult,
        op: Pair<Rule>,
        rhs: ExprResult,
        env: &Environment,
    ) -> ExprResult {
        let (line, column) = op.line_col();
        let op = BinOp::build(op)?;
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
}

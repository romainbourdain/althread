use pest::iterators::Pair;

use crate::{
    ast::{
        expr::{Expr, ExprKind},
        token::{datatype::DataType, unary_op::UnOp},
    },
    env::Environment,
    error::{AlthreadError, ErrorType},
    parser::Rule,
};

use super::{primary::PrimaryExpr, ExprResult};

#[derive(Debug)]
pub struct UnExpr {
    pub op: UnOp,
    pub rhs: Box<Expr>,
    pub line: usize,
    pub column: usize,
}

impl UnExpr {
    pub fn from_pair(op: Pair<Rule>, rhs: ExprResult, env: &Environment) -> ExprResult {
        let (line, column) = op.line_col();
        let op = match op.as_rule() {
            Rule::not => UnOp::Not,
            Rule::neg => UnOp::Neg,
            rule => unreachable!("{:?}", rule),
        };
        let expr = Self {
            line,
            column,
            op,
            rhs: Box::new(rhs?),
        };

        expr.get_datatype(env)?;

        Ok(Expr {
            kind: ExprKind::Unary(expr),
            line,
            column,
        })
    }

    pub fn get_datatype(&self, env: &Environment) -> Result<DataType, AlthreadError> {
        let rhs_type = self.rhs.get_datatype(env)?;

        match self.op {
            UnOp::Not => {
                if rhs_type != DataType::Bool {
                    // TODO : implement error with line and col
                    return Err(AlthreadError::error(
                        ErrorType::TypeError,
                        self.line,
                        self.column,
                        format!("Cannot make {} operation for {}", self.op, rhs_type),
                    ));
                }
                Ok(DataType::Bool)
            }
            UnOp::Neg => {
                if (rhs_type != DataType::Int) && (rhs_type != DataType::Float) {
                    // TODO : implement error with line and col
                    return Err(AlthreadError::error(
                        ErrorType::TypeError,
                        self.line,
                        self.column,
                        format!("Cannot make {} operation for {}", self.op, rhs_type),
                    ));
                }
                Ok(rhs_type)
            }
        }
    }

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

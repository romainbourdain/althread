use std::fmt;

use pest::iterators::Pair;

use crate::{
    ast::datatype::DataType,
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

impl BinOp {
    pub fn build(pair: Pair<Rule>) -> Result<Self, AlthreadError> {
        Ok(match pair.as_rule() {
            Rule::add => BinOp::Add,
            Rule::sub => BinOp::Sub,
            Rule::mul => BinOp::Mul,
            Rule::div => BinOp::Div,
            Rule::modulo => BinOp::Mod,
            Rule::eq => BinOp::Eq,
            Rule::ne => BinOp::Ne,
            Rule::gt => BinOp::Gt,
            Rule::ge => BinOp::Ge,
            Rule::lt => BinOp::Lt,
            Rule::le => BinOp::Le,
            Rule::and => BinOp::And,
            Rule::or => BinOp::Or,
            rule => unreachable!("{:?}", rule),
        })
    }
}

#[derive(Debug, Clone)]
pub enum BinOp {
    Add,
    Sub,
    Mul,
    Div,
    Mod,
    Eq,
    Ne,
    Gt,
    Ge,
    Lt,
    Le,
    And,
    Or,
}

impl fmt::Display for BinOp {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use BinOp::*;
        let op = match self {
            Add => "+",
            Sub => "-",
            Mul => "*",
            Div => "/",
            Mod => "%",
            Eq => "==",
            Ne => "!=",
            Gt => ">",
            Ge => ">=",
            Lt => "<",
            Le => "<=",
            And => "&&",
            Or => "||",
        };
        write!(f, "{}", op)
    }
}

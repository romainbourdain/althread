use pest::iterators::Pair;

use crate::{error::AlthreadError, parser::Rule};

use super::expr::{BinOp, Expr, UnOp};

#[derive(Debug, PartialEq)]
pub enum DataType {
    Int,
    Float,
    Bool,
    String,
    Void,
}

impl DataType {
    pub fn build(pair: Pair<Rule>) -> Result<Self, AlthreadError> {
        match pair.as_str() {
            "int" => Ok(Self::Int),
            "float" => Ok(Self::Float),
            "bool" => Ok(Self::Bool),
            "string" => Ok(Self::String),
            "void" => Ok(Self::Void),
            _ => unreachable!(),
        }
    }

    pub fn from_expr(expr: &Expr) -> Result<Self, AlthreadError> {
        match expr {
            Expr::Int(_) => Ok(DataType::Int),
            Expr::Float(_) => Ok(DataType::Float),
            Expr::Bool(_) => Ok(DataType::Bool),
            Expr::String(_) => Ok(DataType::String),
            Expr::Null => Ok(DataType::Void),
            Expr::Identifier(_) => Ok(DataType::Void),
            Expr::BinOp { op, lhs, rhs } => Self::from_bin_expr(op, lhs, rhs),
            Expr::UnOp { op, lhs } => Self::from_un_expr(op, lhs),
        }
    }

    pub fn from_bin_expr(op: &BinOp, lhs: &Expr, rhs: &Expr) -> Result<Self, AlthreadError> {
        let lhs_type = Self::from_expr(lhs)?;
        let rhs_type = Self::from_expr(rhs)?;
        if lhs_type != rhs_type {
            return Err(AlthreadError::error(0, 0, "Mismatched types".to_string()));
        }

        match op {
            BinOp::Add | BinOp::Sub | BinOp::Mul | BinOp::Div => {
                if (lhs_type != DataType::Int) && (lhs_type != DataType::Float) {
                    return Err(AlthreadError::error(0, 0, "Invalid types".to_string()));
                }
                Ok(lhs_type)
            }
            BinOp::Eq | BinOp::Ne | BinOp::Gt | BinOp::Ge | BinOp::Lt | BinOp::Le => {
                if (lhs_type != DataType::Int) && (lhs_type != DataType::Float) {
                    return Err(AlthreadError::error(0, 0, "Invalid types".to_string()));
                }
                Ok(DataType::Bool)
            }
            BinOp::And | BinOp::Or => {
                if lhs_type != DataType::Bool {
                    return Err(AlthreadError::error(0, 0, "Invalid types".to_string()));
                }
                Ok(DataType::Bool)
            }
        }
    }

    pub fn from_un_expr(op: &UnOp, lhs: &Expr) -> Result<Self, AlthreadError> {
        let lhs_type = Self::from_expr(lhs)?;

        match op {
            UnOp::Not => {
                if lhs_type != DataType::Bool {
                    return Err(AlthreadError::error(0, 0, "Invalid types".to_string()));
                }
                Ok(DataType::Bool)
            }
            UnOp::Neg => {
                if (lhs_type != DataType::Int) && (lhs_type != DataType::Float) {
                    return Err(AlthreadError::error(0, 0, "Invalid types".to_string()));
                }
                Ok(lhs_type)
            }
        }
    }
}

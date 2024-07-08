use pest::iterators::Pair;

use crate::{env::Environment, error::AlthreadError, parser::Rule};

use super::expr::{BinOp, Expr, ExprKind, PrimaryExpr, UnOp};

#[derive(Debug, PartialEq, Clone)]
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

    pub fn from_expr(expr: &ExprKind, env: &Environment) -> Result<Self, String> {
        match expr {
            ExprKind::Primary(expr) => Self::from_primary_expr(expr, env),
            ExprKind::Binary(expr) => Self::from_bin_expr(&expr.lhs, &expr.op, &expr.rhs, env),
            ExprKind::Unary(expr) => Self::from_un_expr(&expr.op, &expr.rhs, env),
        }
    }

    pub fn from_primary_expr(expr: &PrimaryExpr, env: &Environment) -> Result<Self, String> {
        match expr {
            PrimaryExpr::Int(_) => Ok(DataType::Int),
            PrimaryExpr::Float(_) => Ok(DataType::Float),
            PrimaryExpr::Bool(_) => Ok(DataType::Bool),
            PrimaryExpr::String(_) => Ok(DataType::String),
            PrimaryExpr::Null => Ok(DataType::Void),
            PrimaryExpr::Identifier(ident) => {
                let symbol = env.get_symbol(ident)?;
                Ok(symbol.datatype.clone())
            }
        }
    }

    pub fn from_bin_expr(
        lhs: &Expr,
        op: &BinOp,
        rhs: &Expr,
        env: &Environment,
    ) -> Result<Self, String> {
        let lhs_type = Self::from_expr(&lhs.kind, env)?;
        let rhs_type = Self::from_expr(&rhs.kind, env)?;
        if lhs_type != rhs_type {
            return Err(format!(
                "Mismatched types: cannot make {:?} operation between {:?} and {:?}",
                op, lhs_type, rhs_type
            ));
        }

        match op {
            BinOp::Add | BinOp::Sub | BinOp::Mul | BinOp::Div | BinOp::Mod => {
                if (lhs_type != DataType::Int) && (lhs_type != DataType::Float) {
                    return Err(format!(
                        "Cannot make {:?} operation between {:?} and {:?}",
                        op, lhs_type, rhs_type
                    ));
                }
                Ok(lhs_type)
            }
            BinOp::Eq | BinOp::Ne | BinOp::Gt | BinOp::Ge | BinOp::Lt | BinOp::Le => {
                if (lhs_type != DataType::Int) && (lhs_type != DataType::Float) {
                    return Err(format!(
                        "Cannot make {:?} operation between {:?} and {:?}",
                        op, lhs_type, rhs_type
                    ));
                }
                Ok(DataType::Bool)
            }
            BinOp::And | BinOp::Or => {
                if lhs_type != DataType::Bool {
                    return Err(format!(
                        "Cannot make {:?} operation between {:?} and {:?}",
                        op, lhs_type, rhs_type
                    ));
                }
                Ok(DataType::Bool)
            }
        }
    }

    pub fn from_un_expr(op: &UnOp, rhs: &Expr, env: &Environment) -> Result<Self, String> {
        let rhs_type = Self::from_expr(&rhs.kind, env)?;

        match op {
            UnOp::Not => {
                if rhs_type != DataType::Bool {
                    return Err(format!("Cannot make {:?} operation for {:?}", op, rhs_type));
                }
                Ok(DataType::Bool)
            }
            UnOp::Neg => {
                if (rhs_type != DataType::Int) && (rhs_type != DataType::Float) {
                    return Err(format!("Cannot make {:?} operation for {:?}", op, rhs_type));
                }
                Ok(rhs_type)
            }
        }
    }
}

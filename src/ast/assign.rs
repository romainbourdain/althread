use pest::iterators::Pairs;

use crate::{env::Environment, error::AlthreadError, parser::Rule};

use super::{
    datatype::DataType,
    expr::{Expr, PrimaryExpr},
};

#[derive(Debug)]
pub enum AssignBinOp {
    Assign,
    AddAssign,
    SubAssign,
    MulAssign,
    DivAssign,
    ModAssign,
}

#[derive(Debug)]
pub struct Assign {
    identifier: String,
    op: AssignBinOp,
    value: Expr,
}

impl Assign {
    pub fn build(pairs: Pairs<Rule>, env: &Environment) -> Result<Self, AlthreadError> {
        let mut assign = Assign {
            identifier: "".to_string(),
            op: AssignBinOp::Assign,
            value: Expr::Primary(PrimaryExpr::Null),
        };

        for pair in pairs {
            match pair.as_rule() {
                Rule::IDENTIFIER => assign.identifier = pair.as_str().to_string(),
                Rule::assign_op => {
                    assign.op = match pair.as_str() {
                        "=" => AssignBinOp::Assign,
                        "+=" => AssignBinOp::AddAssign,
                        "-=" => AssignBinOp::SubAssign,
                        "*=" => AssignBinOp::MulAssign,
                        "/=" => AssignBinOp::DivAssign,
                        "%=" => AssignBinOp::ModAssign,
                        _ => unreachable!(),
                    }
                }
                Rule::expr => assign.value = Expr::build(pair.into_inner(), env)?,
                Rule::assign_unary_op => {
                    assign.op = match pair.as_str() {
                        "++" => AssignBinOp::AddAssign,
                        "--" => AssignBinOp::SubAssign,
                        _ => unreachable!(),
                    };
                    assign.value = Expr::Primary(PrimaryExpr::Int(1));
                }
                _ => unreachable!(),
            }
        }

        let value_type = DataType::from_expr(&assign.value, env)?;
        let symbol = env.get_symbol(&assign.identifier)?;

        if !symbol.mutable {
            return Err(AlthreadError::error(
                0,
                0,
                "Cannot assign to immutable variable".to_string(),
            ));
        }

        if symbol.datatype != value_type {
            return Err(AlthreadError::error(
                0,
                0,
                "Unexpected type in assignment".to_string(),
            ));
        }

        Ok(assign)
    }
}

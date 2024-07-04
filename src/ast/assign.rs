use pest::iterators::Pairs;

use crate::{env::Environment, error::AlthreadError, parser::Rule};

use super::{
    datatype::DataType,
    expr::{Expr, PrimaryExpr},
};

#[derive(Debug)]
pub enum AssignOp {
    Assign,
    AddAssign,
    SubAssign,
    MulAssign,
    DivAssign,
    ModAssign,
}

#[derive(Debug)]
pub struct Assign {
    pub identifier: String,
    pub op: AssignOp,
    pub value: Expr,
}

impl Assign {
    pub fn build(pairs: Pairs<Rule>, env: &Environment) -> Result<Self, AlthreadError> {
        let mut assign = Assign {
            identifier: "".to_string(),
            op: AssignOp::Assign,
            value: Expr::Primary(PrimaryExpr::Null),
        };

        for pair in pairs {
            match pair.as_rule() {
                Rule::IDENTIFIER => assign.identifier = pair.as_str().to_string(),
                Rule::assign_op => {
                    assign.op = match pair.as_str() {
                        "=" => AssignOp::Assign,
                        "+=" => AssignOp::AddAssign,
                        "-=" => AssignOp::SubAssign,
                        "*=" => AssignOp::MulAssign,
                        "/=" => AssignOp::DivAssign,
                        "%=" => AssignOp::ModAssign,
                        _ => unreachable!(),
                    }
                }
                Rule::expr => assign.value = Expr::build(pair.into_inner(), env)?,
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

use std::fmt;

use pest::iterators::Pair;

use crate::{
    env::Environment,
    error::{AlthreadError, ErrorType},
    parser::Rule,
};

use super::{
    datatype::DataType,
    expr::{primary::PrimaryExpr, Expr, ExprKind},
};

#[derive(Debug)]
pub struct Assign {
    pub identifier: String,
    pub op: AssignOp,
    pub value: Expr,
    pub line: usize,
    pub column: usize,
}

impl Assign {
    pub fn new(line: usize, column: usize) -> Self {
        Self {
            identifier: "".to_string(),
            op: AssignOp::Assign,
            value: Expr::new(ExprKind::Primary(PrimaryExpr::Null)),
            line,
            column,
        }
    }
}

impl Assign {
    pub fn build(pair: Pair<Rule>, env: &Environment) -> Result<Self, AlthreadError> {
        let (line, column) = pair.line_col();
        let mut assign = Self::new(line, column);

        for pair in pair.into_inner() {
            match pair.as_rule() {
                Rule::IDENTIFIER => assign.identifier = pair.as_str().to_string(),
                Rule::assign_op => assign.op = AssignOp::build(pair)?,
                Rule::expr => assign.value = Expr::build(pair, env)?,
                Rule::assign_unary_op => {
                    assign.op = AssignOp::build(pair)?;
                    assign.value = Expr::new(ExprKind::Primary(PrimaryExpr::Int(1)));
                }
                _ => unreachable!(),
            }
        }

        let value_type = assign.value.get_datatype(env)?;

        match (&assign.op, &value_type) {
            (AssignOp::Assign, _) => {}
            (AssignOp::AddAssign, DataType::Int) => {}
            (AssignOp::SubAssign, DataType::Int) => {}
            (AssignOp::MulAssign, DataType::Int) => {}
            (AssignOp::DivAssign, DataType::Int) => {}
            (AssignOp::ModAssign, DataType::Int) => {}
            (op, datatype) => {
                return Err(AlthreadError::error(
                    ErrorType::TypeError,
                    line,
                    column,
                    format!("Unexpected {op} operation with {datatype}"),
                ))
            }
        }

        let symbol = env.get_symbol(&assign.identifier).map_err(|e| {
            AlthreadError::error(
                ErrorType::VariableError,
                assign.value.line,
                assign.value.column,
                e,
            )
        })?;

        if !symbol.mutable {
            return Err(AlthreadError::error(
                ErrorType::VariableError,
                assign.value.line,
                assign.value.column,
                "Cannot change immutable variable value".to_string(),
            ));
        }

        if symbol.datatype != value_type {
            return Err(AlthreadError::error(
                ErrorType::TypeError,
                assign.line,
                assign.column,
                format!(
                    "Cannot change {} type from {} to {}",
                    assign.identifier, symbol.datatype, value_type
                ),
            ));
        }

        Ok(assign)
    }
}

impl AssignOp {
    pub fn build(pair: Pair<Rule>) -> Result<Self, AlthreadError> {
        Ok(match pair.as_str() {
            "=" => AssignOp::Assign,
            "+=" => AssignOp::AddAssign,
            "-=" => AssignOp::SubAssign,
            "*=" => AssignOp::MulAssign,
            "/=" => AssignOp::DivAssign,
            "%=" => AssignOp::ModAssign,
            "++" => AssignOp::AddAssign,
            "--" => AssignOp::SubAssign,
            _ => unimplemented!(),
        })
    }
}

#[derive(Debug, PartialEq)]
pub enum AssignOp {
    Assign,
    AddAssign,
    SubAssign,
    MulAssign,
    DivAssign,
    ModAssign,
}

impl fmt::Display for AssignOp {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use AssignOp::*;
        let op = match self {
            Assign => "=",
            AddAssign => "+=",
            SubAssign => "-=",
            MulAssign => "*=",
            DivAssign => "/=",
            ModAssign => "%=",
        };
        write!(f, "{}", op)
    }
}

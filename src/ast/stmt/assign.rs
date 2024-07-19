use pest::iterators::Pair;

use crate::{
    ast::{
        expr::{primary::PrimaryExpr, Expr, ExprKind},
        token::{assign_op::AssignOp, datatype::DataType},
    },
    env::Environment,
    error::{AlthreadError, ErrorType},
    parser::Rule,
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

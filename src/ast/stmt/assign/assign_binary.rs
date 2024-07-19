use pest::iterators::Pair;

use crate::{
    ast::{
        expr::{primary::PrimaryExpr, Expr, ExprKind},
        token::{binary_assign_op::BinaryAssignOp, datatype::DataType},
    },
    env::Environment,
    error::{AlthreadError, ErrorType},
    parser::Rule,
};

#[derive(Debug)]
pub struct AssignBinary {
    pub left: String,
    pub op: BinaryAssignOp,
    pub right: Expr,
    pub line: usize,
    pub column: usize,
}

impl AssignBinary {
    pub fn new(line: usize, column: usize) -> Self {
        Self {
            left: "".to_string(),
            op: BinaryAssignOp::Assign,
            right: Expr::new(ExprKind::Primary(PrimaryExpr::Null)),
            line,
            column,
        }
    }
}

impl AssignBinary {
    pub fn build(pair: Pair<Rule>, env: &Environment) -> Result<Self, AlthreadError> {
        let (line, column) = pair.line_col();
        let mut assign = Self::new(line, column);

        for pair in pair.into_inner() {
            match pair.as_rule() {
                Rule::IDENTIFIER => assign.left = pair.as_str().to_string(),
                Rule::assign_op => assign.op = BinaryAssignOp::build(pair)?,
                Rule::expr => assign.right = Expr::build(pair, env)?,
                Rule::assign_unary_op => {
                    assign.op = BinaryAssignOp::build(pair)?;
                    assign.right = Expr::new(ExprKind::Primary(PrimaryExpr::Int(1)));
                }
                _ => unreachable!(),
            }
        }

        let value_type = assign.right.get_datatype(env)?;

        match (&assign.op, &value_type) {
            (BinaryAssignOp::Assign, _) => {}
            (BinaryAssignOp::AddAssign, DataType::Int) => {}
            (BinaryAssignOp::SubAssign, DataType::Int) => {}
            (BinaryAssignOp::MulAssign, DataType::Int) => {}
            (BinaryAssignOp::DivAssign, DataType::Int) => {}
            (BinaryAssignOp::ModAssign, DataType::Int) => {}
            (op, datatype) => {
                return Err(AlthreadError::error(
                    ErrorType::TypeError,
                    line,
                    column,
                    format!("Unexpected {op} operation with {datatype}"),
                ))
            }
        }

        let symbol = env.get_symbol(&assign.left).map_err(|e| {
            AlthreadError::error(
                ErrorType::VariableError,
                assign.right.line,
                assign.right.column,
                e,
            )
        })?;

        if !symbol.mutable {
            return Err(AlthreadError::error(
                ErrorType::VariableError,
                assign.right.line,
                assign.right.column,
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
                    assign.left, symbol.datatype, value_type
                ),
            ));
        }

        Ok(assign)
    }
}

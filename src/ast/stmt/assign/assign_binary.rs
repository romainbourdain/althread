use pest::iterators::Pair;

use crate::{
    ast::{
        expr::{primary::PrimaryExpr, Expr, ExprKind},
        token::{assign_binary_op::AssignBinaryOp, datatype::DataType},
    },
    env::Environment,
    error::{AlthreadError, ErrorType},
    parser::Rule,
};

#[derive(Debug)]
pub struct AssignBinary {
    pub left: String,
    pub op: AssignBinaryOp,
    pub right: Expr,
    pub line: usize,
    pub column: usize,
}

impl AssignBinary {
    pub fn new(line: usize, column: usize) -> Self {
        Self {
            left: "".to_string(),
            op: AssignBinaryOp::Assign,
            right: Expr::new(ExprKind::Primary(PrimaryExpr::Null)),
            line,
            column,
        }
    }
}

impl AssignBinary {
    pub fn from_pair(pair: Pair<Rule>, env: &Environment) -> Result<Self, AlthreadError> {
        let (line, column) = pair.line_col();
        let mut assign = Self::new(line, column);

        for pair in pair.into_inner() {
            match pair.as_rule() {
                Rule::IDENTIFIER => assign.left = pair.as_str().to_string(),
                Rule::assign_op => assign.op = AssignBinaryOp::from_pair(pair)?,
                Rule::expr => assign.right = Expr::from_pair(pair, env)?,
                Rule::assign_unary_op => {
                    assign.op = AssignBinaryOp::from_pair(pair)?;
                    assign.right = Expr::new(ExprKind::Primary(PrimaryExpr::Int(1)));
                }
                _ => unreachable!(),
            }
        }

        let value_type = assign.right.get_datatype(env)?;

        match (&assign.op, &value_type) {
            (AssignBinaryOp::Assign, _) => {}
            (AssignBinaryOp::AddAssign, DataType::Int) => {}
            (AssignBinaryOp::SubAssign, DataType::Int) => {}
            (AssignBinaryOp::MulAssign, DataType::Int) => {}
            (AssignBinaryOp::DivAssign, DataType::Int) => {}
            (AssignBinaryOp::ModAssign, DataType::Int) => {}
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

    pub fn eval(&self, env: &mut Environment) -> Result<(), AlthreadError> {
        let symbol = env.get_symbol(&self.left).map_err(|e| {
            AlthreadError::error(ErrorType::VariableError, self.line, self.column, e)
        })?;
        if let Some(symbol_value) = &symbol.value {
            let value = match self.op {
                AssignBinaryOp::Assign => self.right.eval(env)?,
                AssignBinaryOp::AddAssign => match (self.right.eval(env)?, symbol_value) {
                    (PrimaryExpr::Int(value), PrimaryExpr::Int(cur)) => {
                        PrimaryExpr::Int(cur + value)
                    }
                    _ => unreachable!(),
                },
                AssignBinaryOp::SubAssign => match (self.right.eval(env)?, symbol_value) {
                    (PrimaryExpr::Int(value), PrimaryExpr::Int(cur)) => {
                        PrimaryExpr::Int(cur - value)
                    }
                    _ => unreachable!(),
                },
                AssignBinaryOp::MulAssign => match (self.right.eval(env)?, symbol_value) {
                    (PrimaryExpr::Int(value), PrimaryExpr::Int(cur)) => {
                        PrimaryExpr::Int(cur * value)
                    }
                    _ => unreachable!(),
                },
                AssignBinaryOp::DivAssign => match (self.right.eval(env)?, symbol_value) {
                    (PrimaryExpr::Int(value), PrimaryExpr::Int(cur)) => {
                        PrimaryExpr::Int(cur / value)
                    }
                    _ => unreachable!(),
                },
                AssignBinaryOp::ModAssign => match (self.right.eval(env)?, symbol_value) {
                    (PrimaryExpr::Int(value), PrimaryExpr::Int(cur)) => {
                        PrimaryExpr::Int(cur % value)
                    }
                    _ => unreachable!(),
                },
            };

            env.update_symbol(&self.left, value).map_err(|e| {
                AlthreadError::error(ErrorType::VariableError, self.line, self.column, e)
            })?;

            Ok(())
        } else {
            unreachable!()
        }
    }
}

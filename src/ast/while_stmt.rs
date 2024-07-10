use pest::iterators::Pair;

use crate::{
    env::Environment,
    error::{AlthreadError, ErrorType},
    nodes::{block::Block, datatype::DataType, expr::Expr, while_stmt::WhileStmt},
    parser::Rule,
};

impl WhileStmt {
    pub fn build(pair: Pair<Rule>, env: &mut Environment) -> Result<Self, AlthreadError> {
        let (line, column) = pair.line_col();
        let mut pairs = pair.into_inner();
        let condition = pairs.next().unwrap();
        let block = pairs.next().unwrap();

        let condition = Expr::build(condition, env)?;
        let block = Block::parse_and_push(block, env)?;

        DataType::from_expr(&condition.kind, env).map_err(|e| {
            AlthreadError::error(
                ErrorType::VariableError,
                condition.line,
                condition.column,
                e,
            )
        })?;

        match DataType::from_expr(&condition.kind, env) {
            Ok(DataType::Bool) => Ok(WhileStmt {
                condition,
                block,
                line,
                column,
            }),
            Ok(datatype) => {
                return Err(AlthreadError::error(
                    ErrorType::TypeError,
                    condition.line,
                    condition.column,
                    format!("While condition must be a boolean, found {}", datatype),
                ));
            }
            Err(e) => {
                return Err(AlthreadError::error(
                    ErrorType::TypeError,
                    condition.line,
                    condition.column,
                    e,
                ))
            }
        }
    }
}

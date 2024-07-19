use pest::iterators::Pair;

use crate::{
    ast::{block::Block, expr::Expr, token::datatype::DataType},
    env::Environment,
    error::{AlthreadError, ErrorType},
    parser::Rule,
};

#[derive(Debug)]
pub struct WhileStmt {
    pub condition: Expr,
    pub block: Block,
    pub line: usize,
    pub column: usize,
}

impl WhileStmt {
    pub fn build(pair: Pair<Rule>, env: &mut Environment) -> Result<Self, AlthreadError> {
        let (line, column) = pair.line_col();
        let mut pairs = pair.into_inner();
        let condition = pairs.next().unwrap();
        let block = pairs.next().unwrap();

        let condition = Expr::build(condition, env)?;
        let block = Block::parse_and_push(block, env)?;

        match condition.get_datatype(env)? {
            DataType::Bool => Ok(WhileStmt {
                condition,
                block,
                line,
                column,
            }),
            datatype => Err(AlthreadError::error(
                ErrorType::TypeError,
                condition.line,
                condition.column,
                format!("While condition must be a boolean, found {}", datatype),
            )),
        }
    }
}

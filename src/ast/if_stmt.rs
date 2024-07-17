use pest::iterators::Pair;

use crate::{
    env::Environment,
    error::{AlthreadError, ErrorType},
    parser::Rule,
};

use super::{block::Block, datatype::DataType, expr::Expr};

#[derive(Debug)]
pub struct IfStmt {
    pub condition: Expr,
    pub block: Block,
    pub else_block: Option<Block>,
    pub line: usize,
    pub column: usize,
}

impl IfStmt {
    pub fn build(pair: Pair<Rule>, env: &mut Environment) -> Result<Self, AlthreadError> {
        let (line, column) = pair.line_col();
        let mut pairs = pair.into_inner();
        let condition = pairs.next().unwrap();
        let block = pairs.next().unwrap();
        let else_block = pairs.next();

        let condition = Expr::build(condition, env)?;
        let block = Block::parse_and_push(block, env)?;
        let else_block = else_block
            .map(|block| Block::parse_and_push(block, env))
            .transpose()?;

        match condition.get_datatype(env)? {
            DataType::Bool => Ok(IfStmt {
                condition,
                block,
                else_block,
                line,
                column,
            }),
            datatype => Err(AlthreadError::error(
                ErrorType::TypeError,
                condition.line,
                condition.column,
                format!("If condition must be a boolean, found {}", datatype),
            )),
        }
    }
}

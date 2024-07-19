use std::io::Write;

use pest::iterators::Pair;

use crate::{
    ast::{
        block::Block,
        expr::{primary::PrimaryExpr, Expr},
        token::datatype::DataType,
    },
    env::Environment,
    error::{AlthreadError, ErrorType},
    parser::Rule,
};

#[derive(Debug)]
pub struct IfStmt {
    pub condition: Expr,
    pub block: Block,
    pub else_block: Option<Block>,
    pub line: usize,
    pub column: usize,
}

impl IfStmt {
    pub fn from_pair(pair: Pair<Rule>, env: &mut Environment) -> Result<Self, AlthreadError> {
        let (line, column) = pair.line_col();
        let mut pairs = pair.into_inner();
        let condition = pairs.next().unwrap();
        let block = pairs.next().unwrap();
        let else_block = pairs.next();

        let condition = Expr::from_pair(condition, env)?;
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

    pub fn eval<W>(&self, env: &mut Environment, output: &mut W) -> Result<(), AlthreadError>
    where
        W: Write,
    {
        match self.condition.eval(env)? {
            PrimaryExpr::Bool(true) => self.block.eval(env, output)?,
            PrimaryExpr::Bool(false) => {
                if let Some(else_block) = &self.else_block {
                    else_block.eval(env, output)?;
                }
            }
            _ => {
                return Err(AlthreadError::error(
                    ErrorType::RuntimeError,
                    self.line,
                    self.column,
                    format!("Condition must be a boolean"),
                ))
            }
        }

        Ok(())
    }
}

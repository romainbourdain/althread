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

    pub fn eval<W>(&self, env: &mut Environment, output: &mut W) -> Result<(), AlthreadError>
    where
        W: Write,
    {
        loop {
            match self.condition.eval(env)? {
                PrimaryExpr::Bool(true) => self.block.eval(env, output)?,
                PrimaryExpr::Bool(false) => break,
                _ => {
                    return Err(AlthreadError::error(
                        ErrorType::RuntimeError,
                        self.line,
                        self.column,
                        format!("Condition must be a boolean"),
                    ))
                }
            }
        }

        Ok(())
    }
}

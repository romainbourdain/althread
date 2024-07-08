use pest::iterators::Pair;

use crate::{
    env::Environment,
    error::{AlthreadError, ErrorType},
    parser::Rule,
};

use super::{datatype::DataType, expr::Expr, stmt::Stmt};

#[derive(Debug)]
pub struct Block {
    pub stmts: Vec<Stmt>,
    pub line: usize,
    pub column: usize,
}

impl Block {
    pub fn new(line: usize, column: usize) -> Self {
        Self {
            stmts: Vec::new(),
            line,
            column,
        }
    }

    pub fn parse(pair: Pair<Rule>, env: &mut Environment) -> Result<Self, AlthreadError> {
        let (line, column) = pair.line_col();
        let mut block = Self::new(line, column);

        for pair in pair.into_inner() {
            block.stmts.push(Stmt::build(pair, env)?);
        }

        Ok(block)
    }

    pub fn parse_and_push(pair: Pair<Rule>, env: &mut Environment) -> Result<Self, AlthreadError> {
        env.push_table();
        let block = Self::parse(pair, env)?;
        env.pop_table();
        Ok(block)
    }
}

#[derive(Debug)]
pub struct IfBlock {
    pub condition: Expr,
    pub block: Block,
    pub else_block: Option<Block>,
    pub line: usize,
    pub column: usize,
}

impl IfBlock {
    pub fn build(pair: Pair<Rule>, env: &mut Environment) -> Result<Self, AlthreadError> {
        let (line, column) = pair.line_col();
        let mut pairs = pair.into_inner();
        let condition = pairs.next().unwrap();
        let block = pairs.next().unwrap();
        let else_block = pairs.next();

        let condition = Expr::build(condition, env)?;
        let block = Block::parse_and_push(block, env)?;
        let else_block = match else_block {
            Some(else_block) => Some(Block::parse_and_push(else_block, env)?),
            None => None,
        };

        match DataType::from_expr(&condition.kind, env) {
            Ok(DataType::Bool) => Ok(IfBlock {
                condition,
                block,
                else_block,
                line,
                column,
            }),
            Ok(datatype) => {
                return Err(AlthreadError::error(
                    ErrorType::TypeError,
                    condition.line,
                    condition.column,
                    format!("If condition must be a boolean, found {}", datatype),
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

#[derive(Debug)]
pub struct WhileBlock {
    pub condition: Expr,
    pub block: Block,
    pub line: usize,
    pub column: usize,
}

impl WhileBlock {
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
            Ok(DataType::Bool) => Ok(WhileBlock {
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

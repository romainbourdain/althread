use pest::iterators::{Pair, Pairs};

use crate::{env::Environment, error::AlthreadError, parser::Rule};

use super::{datatype::DataType, expr::Expr, stmt::Stmt};

pub type Block = Vec<Stmt>;

pub fn parse_block(pairs: Pairs<Rule>, env: &mut Environment) -> Result<Block, AlthreadError> {
    env.push_table();
    let stmts = parse_shared_block(pairs, env)?;
    env.pop_table();
    Ok(stmts)
}

pub fn parse_shared_block(
    pairs: Pairs<Rule>,
    env: &mut Environment,
) -> Result<Block, AlthreadError> {
    let mut stmts = Vec::new();

    for pair in pairs {
        stmts.push(Stmt::build(pair, env)?);
    }

    Ok(stmts)
}

#[derive(Debug)]
pub struct IfBlock {
    pub condition: Expr,
    pub block: Block,
    pub else_block: Option<Block>,
}

impl IfBlock {
    pub fn build(pairs: Pairs<Rule>, env: &mut Environment) -> Result<Self, AlthreadError> {
        let mut pairs = pairs.clone();
        let condition = pairs.next().unwrap();
        let block = pairs.next().unwrap();
        let else_block = pairs.next();

        let condition = Expr::build(condition.into_inner(), env)?;
        let block = parse_block(block.into_inner(), env)?;
        let else_block = match else_block {
            Some(else_block) => Some(parse_block(else_block.into_inner(), env)?),
            None => None,
        };

        match DataType::from_expr(&condition, env)? {
            DataType::Bool => Ok(IfBlock {
                condition,
                block,
                else_block,
            }),
            _ => {
                return Err(AlthreadError::error(
                    0,
                    0,
                    "If condition must be a boolean".to_string(),
                ));
            }
        }
    }
}

#[derive(Debug)]
pub struct WhileBlock {
    pub condition: Expr,
    pub block: Block,
}

impl WhileBlock {
    pub fn build(pairs: Pairs<Rule>, env: &mut Environment) -> Result<Self, AlthreadError> {
        let mut pairs = pairs.clone();
        let condition = pairs.next().unwrap();
        let block = pairs.next().unwrap();

        let condition = Expr::build(condition.into_inner(), env)?;
        let block = parse_block(block.into_inner(), env)?;

        DataType::from_expr(&condition, env)?;

        Ok(WhileBlock { condition, block })
    }
}

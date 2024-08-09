use pest::iterators::Pair;

use crate::{
    ast::{
        node::expr::primary_expr::Identifier,
        token::{FromPair, Token},
    },
    error::AlthreadResult,
    no_rule,
    parser::Rule,
};

#[derive(Debug)]
pub struct UnaryAssign {
    pub identifier: Token<Identifier>,
    pub operator: Token<UnaryAssignOp>,
    pub line: usize,
    pub column: usize,
}

impl UnaryAssign {
    pub fn build(pair: Pair<Rule>) -> AlthreadResult<Self> {
        let (line, column) = pair.line_col();
        let mut pairs = pair.into_inner();

        let identifier = Token::build(pairs.next().unwrap())?;
        let operator = Token::build(pairs.next().unwrap())?;

        Ok(Self {
            identifier,
            operator,
            line,
            column,
        })
    }
}

#[derive(Debug)]
pub enum UnaryAssignOp {
    Increment,
    Decrement,
}

impl FromPair for UnaryAssignOp {
    fn from_pair(pair: Pair<Rule>) -> AlthreadResult<Self> {
        match pair.as_str() {
            "++" => Ok(Self::Increment),
            "--" => Ok(Self::Decrement),
            _ => Err(no_rule!(pair)),
        }
    }
}

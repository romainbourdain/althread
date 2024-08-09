use pest::iterators::Pair;

use crate::{
    ast::{
        node::expr::{primary_expr::Identifier, Expr},
        token::{FromPair, Token},
    },
    error::AlthreadResult,
    no_rule,
    parser::Rule,
};

#[derive(Debug)]
pub struct BinaryAssign {
    pub identifier: Token<Identifier>,
    pub operator: Token<BinaryAssignOp>,
    pub value: Expr,
    pub line: usize,
    pub column: usize,
}

impl BinaryAssign {
    pub fn build(pair: Pair<Rule>) -> AlthreadResult<Self> {
        let (line, column) = pair.line_col();
        let mut pairs = pair.into_inner();

        let identifier = Token::build(pairs.next().unwrap())?;
        let operator = Token::build(pairs.next().unwrap())?;
        let value = Expr::build(pairs.next().unwrap())?;

        Ok(Self {
            identifier,
            operator,
            value,
            line,
            column,
        })
    }
}

#[derive(Debug)]
pub enum BinaryAssignOp {
    Assign,
    AddAssign,
    SubAssign,
    MulAssign,
    DivAssign,
    ModAssign,
}

impl FromPair for BinaryAssignOp {
    fn from_pair(pair: Pair<Rule>) -> AlthreadResult<Self> {
        match pair.as_str() {
            "=" => Ok(Self::Assign),
            "+=" => Ok(Self::AddAssign),
            "-=" => Ok(Self::SubAssign),
            "*=" => Ok(Self::MulAssign),
            "/=" => Ok(Self::DivAssign),
            "%=" => Ok(Self::ModAssign),
            _ => Err(no_rule!(pair)),
        }
    }
}

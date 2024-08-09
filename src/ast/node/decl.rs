use pest::iterators::Pair;

use crate::{
    ast::{
        datatype::DataType,
        token::{FromPair, Token},
    },
    error::AlthreadResult,
    no_rule,
    parser::Rule,
};

use super::expr::{primary_expr::Identifier, Expr};

#[derive(Debug)]
pub struct Decl {
    pub keyword: Token<DeclKeyword>,
    pub identifier: Token<Identifier>,
    pub datatype: Option<Token<DataType>>,
    pub value: Option<Expr>,
    pub line: usize,
    pub column: usize,
}

impl Decl {
    pub fn build(pair: Pair<Rule>) -> AlthreadResult<Self> {
        let (line, column) = pair.line_col();
        let mut pairs = pair.into_inner();

        let keyword = Token::build(pairs.next().unwrap())?;
        let identifier = Token::build(pairs.next().unwrap())?;
        let mut datatype = None;
        let mut value = None;

        for pair in pairs {
            match pair.as_rule() {
                Rule::DATATYPE => {
                    datatype = Some(Token::build(pair)?);
                }
                Rule::expr => {
                    value = Some(Expr::build(pair)?);
                }
                _ => return Err(no_rule!(pair)),
            }
        }

        Ok(Self {
            keyword,
            identifier,
            datatype,
            value,
            line,
            column,
        })
    }
}

#[derive(Debug)]
pub enum DeclKeyword {
    Let,
    Const,
}

impl FromPair for DeclKeyword {
    fn from_pair(pair: Pair<Rule>) -> AlthreadResult<Self> {
        match pair.as_str() {
            "let" => Ok(Self::Let),
            "const" => Ok(Self::Const),
            _ => Err(no_rule!(pair)),
        }
    }
}

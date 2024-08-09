use pest::iterators::Pair;

use crate::{ast::token::Token, error::AlthreadResult, parser::Rule};

use super::expr::primary_expr::Identifier;

#[derive(Debug)]
pub struct Run {
    pub identifier: Token<Identifier>,
    pub line: usize,
    pub column: usize,
}

impl Run {
    pub fn build(pair: Pair<Rule>) -> AlthreadResult<Self> {
        let (line, column) = pair.line_col();
        let mut pairs = pair.into_inner();

        let identifier = Token::build(pairs.next().unwrap())?;

        Ok(Self {
            identifier,
            line,
            column,
        })
    }
}

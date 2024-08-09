use pest::iterators::Pair;

use crate::{error::AlthreadResult, parser::Rule};

#[derive(Debug, Clone)]
pub struct Token<T> {
    pub value: T,
    pub line: usize,
    pub column: usize,
}

pub trait FromPair: Sized {
    fn from_pair(pair: Pair<Rule>) -> AlthreadResult<Self>;
}

impl<T: FromPair> Token<T> {
    pub fn build(pair: Pair<Rule>) -> AlthreadResult<Self> {
        let (line, col) = pair.line_col();
        Ok(Token {
            value: T::from_pair(pair)?,
            line,
            column: col,
        })
    }
}

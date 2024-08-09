use std::fmt;
use std::fmt::Formatter;

use pest::iterators::Pair;

use crate::{error::AlthreadResult, parser::Rule};

#[derive(Debug, Clone)]
pub struct Node<T> {
    pub value: T,
    pub line: usize,
    pub column: usize,
}

pub trait Build: Sized {
    fn build(pair: Pair<Rule>) -> AlthreadResult<Self>;
}

impl<T: Build> Node<T> {
    pub fn build(pair: Pair<Rule>) -> AlthreadResult<Self> {
        let (line, col) = pair.line_col();
        Ok(Node {
            value: T::build(pair)?,
            line,
            column: col,
        })
    }
}

impl<T: fmt::Display> fmt::Display for Node<T> {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "{}", self.value)
    }
}

use std::fmt;

use pest::iterators::{Pair, Pairs};

use crate::{env::process_env::ProcessEnv, error::AlthreadResult, parser::Rule};

use super::{
    display::{AstDisplay, Prefix},
    token::literal::Literal,
};

#[derive(Debug, Clone)]
pub struct Node<T> {
    pub value: T,
    pub line: usize,
    pub column: usize,
}

pub trait NodeBuilder: Sized {
    fn build(pairs: Pairs<Rule>) -> AlthreadResult<Self>;
}

pub trait NodeExecutor: Sized {
    fn eval(&self, env: &mut ProcessEnv) -> AlthreadResult<Option<Literal>>;
}

impl<T: NodeBuilder> Node<T> {
    pub fn build(pair: Pair<Rule>) -> AlthreadResult<Self> {
        let (line, col) = pair.line_col();
        Ok(Node {
            value: T::build(pair.into_inner())?,
            line,
            column: col,
        })
    }
}

impl<T: NodeExecutor> Node<T> {
    pub fn eval(&self, env: &mut ProcessEnv) -> AlthreadResult<Option<Literal>> {
        self.value.eval(env)
    }
}

impl<T: AstDisplay> AstDisplay for Node<T> {
    fn ast_fmt(&self, f: &mut fmt::Formatter, prefix: &Prefix) -> fmt::Result {
        self.value.ast_fmt(f, prefix)
    }
}

impl<T: fmt::Display> fmt::Display for Node<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.value)
    }
}

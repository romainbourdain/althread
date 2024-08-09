use std::fmt;

use pest::iterators::Pair;

use crate::{ast::node::Build, error::AlthreadResult, no_rule, parser::Rule};

#[derive(Debug)]
pub enum DeclKeyword {
    Let,
    Const,
}

impl Build for DeclKeyword {
    fn build(pair: Pair<Rule>) -> AlthreadResult<Self> {
        match pair.as_str() {
            "let" => Ok(Self::Let),
            "const" => Ok(Self::Const),
            _ => Err(no_rule!(pair)),
        }
    }
}

impl fmt::Display for DeclKeyword {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Let => write!(f, "let"),
            Self::Const => write!(f, "const"),
        }
    }
}

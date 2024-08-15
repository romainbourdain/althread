use std::fmt;

use pest::iterators::Pairs;

use crate::{ast::node::NodeBuilder, error::AlthreadResult, no_rule, parser::Rule};

#[derive(Debug, PartialEq)]
pub enum DeclarationKeyword {
    Let,
    Const,
}

impl NodeBuilder for DeclarationKeyword {
    fn build(mut pairs: Pairs<Rule>) -> AlthreadResult<Self> {
        let pair = pairs.next().unwrap();
        match pair.as_rule() {
            Rule::LET_KW => Ok(Self::Let),
            Rule::CONST_KW => Ok(Self::Const),
            _ => Err(no_rule!(pair)),
        }
    }
}

impl fmt::Display for DeclarationKeyword {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Let => write!(f, "let"),
            Self::Const => write!(f, "const"),
        }
    }
}

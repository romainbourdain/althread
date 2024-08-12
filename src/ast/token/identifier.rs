use pest::iterators::Pair;

use crate::{ast::node::Token, error::AlthreadResult, no_rule, parser::Rule};

pub type Identifier = String;

impl Token for Identifier {
    fn build(pair: Pair<Rule>) -> AlthreadResult<Self> {
        match pair.as_rule() {
            Rule::IDENTIFIER => Ok(pair.as_str().to_string()),
            _ => Err(no_rule!(pair)),
        }
    }
}

use pest::iterators::Pair;

use crate::{ast::node::Build, error::AlthreadResult, no_rule, parser::Rule};

pub type Identifier = String;

impl Build for Identifier {
    fn build(pair: Pair<Rule>) -> AlthreadResult<Self> {
        match pair.as_rule() {
            Rule::IDENTIFIER => Ok(pair.as_str().to_string()),
            _ => Err(no_rule!(pair)),
        }
    }
}

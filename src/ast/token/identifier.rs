use pest::iterators::Pairs;

use crate::{ast::node::Build, error::AlthreadResult, no_rule, parser::Rule};

pub type Identifier = String;

impl Build for Identifier {
    fn build(mut pairs: Pairs<Rule>) -> AlthreadResult<Self> {
        let pair = pairs.next().unwrap();
        match pair.as_rule() {
            Rule::IDENT => Ok(pair.as_str().to_string()),
            _ => Err(no_rule!(pair)),
        }
    }
}

use pest::iterators::Pairs;

use crate::{
    ast::node::{NodeBuilder, NodeExecutor},
    env::Env,
    error::AlthreadResult,
    no_rule,
    parser::Rule,
};

use super::literal::Literal;

pub type Identifier = String;

impl NodeBuilder for Identifier {
    fn build(mut pairs: Pairs<Rule>) -> AlthreadResult<Self> {
        let pair = pairs.next().unwrap();
        match pair.as_rule() {
            Rule::IDENT => Ok(pair.as_str().to_string()),
            _ => Err(no_rule!(pair)),
        }
    }
}

impl NodeExecutor for Identifier {
    fn eval(&self, _env: &mut Env) -> AlthreadResult<Option<Literal>> {
        // TODO: Get the value of the identifier from the environment
        Ok(Some(Literal::Null))
    }
}

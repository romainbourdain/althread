pub mod binary_assign;
pub mod unary_assign;

use binary_assign::BinaryAssign;
use pest::iterators::Pair;
use unary_assign::UnaryAssign;

use crate::{error::AlthreadResult, no_rule, parser::Rule};

#[derive(Debug)]
pub enum Assign {
    Binary(BinaryAssign),
    Unary(UnaryAssign),
}

impl Assign {
    pub fn build(pair: Pair<Rule>) -> AlthreadResult<Self> {
        let pair = pair.into_inner().next().unwrap();

        match pair.as_rule() {
            Rule::assign_unary => Ok(Self::Unary(UnaryAssign::build(pair)?)),
            Rule::assign_binary => Ok(Self::Binary(BinaryAssign::build(pair)?)),
            _ => Err(no_rule!(pair)),
        }
    }
}

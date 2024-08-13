use std::fmt;

use pest::iterators::Pairs;

use crate::{ast::node::NodeBuilder, error::AlthreadResult, no_rule, parser::Rule};

#[derive(Debug)]
pub enum UnaryOperator {
    Positive,
    Negative,
    Not,
}

impl NodeBuilder for UnaryOperator {
    fn build(mut pairs: Pairs<Rule>) -> AlthreadResult<Self> {
        let pair = pairs.next().unwrap();
        match pair.as_rule() {
            Rule::POS_OP => Ok(Self::Positive),
            Rule::NEG_OP => Ok(Self::Negative),
            Rule::NOT_OP => Ok(Self::Not),
            _ => Err(no_rule!(pair)),
        }
    }
}

impl fmt::Display for UnaryOperator {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let op = match self {
            UnaryOperator::Positive => "+",
            UnaryOperator::Negative => "-",
            UnaryOperator::Not => "!",
        };

        write!(f, "{}", op)
    }
}

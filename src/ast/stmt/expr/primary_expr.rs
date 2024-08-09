use std::str::FromStr;

use pest::iterators::Pair;

use crate::{
    ast::node::Build,
    error::{AlthreadError, AlthreadResult, ErrorType},
    no_rule,
    parser::Rule,
};

pub type Identifier = String;

impl Build for Identifier {
    fn build(pair: Pair<Rule>) -> AlthreadResult<Self> {
        match pair.as_rule() {
            Rule::IDENTIFIER => Ok(pair.as_str().to_string()),
            _ => Err(no_rule!(pair)),
        }
    }
}

#[derive(Debug)]
pub enum PrimaryExpr {
    Null(),
    Bool(bool),
    Int(i64),
    Float(f64),
    String(String),
    Identifier(Identifier),
}

impl Build for PrimaryExpr {
    fn build(pair: Pair<Rule>) -> AlthreadResult<Self> {
        fn parse_with_error<T: FromStr>(pair: Pair<Rule>) -> AlthreadResult<T> {
            let (line, col) = pair.line_col();
            pair.as_str().parse::<T>().map_err(|_| {
                AlthreadError::new(
                    ErrorType::SyntaxError,
                    line,
                    col,
                    format!("Cannot parse {}", pair.as_str()),
                )
            })
        }

        match pair.as_rule() {
            Rule::NULL => Ok(Self::Null()),
            Rule::BOOLEAN => Ok(Self::Bool(parse_with_error::<bool>(pair)?)),
            Rule::INTEGER => Ok(Self::Int(parse_with_error::<i64>(pair)?)),
            Rule::FLOAT => Ok(Self::Float(parse_with_error::<f64>(pair)?)),
            Rule::STRING => Ok(Self::String(pair.as_str().to_string())),
            Rule::IDENTIFIER => Ok(Self::Identifier(Identifier::build(pair)?)),
            _ => Err(no_rule!(pair)),
        }
    }
}

use pest::iterators::Pair;

use crate::{error::AlthreadError, nodes::datatype::DataType, parser::Rule};

impl DataType {
    pub fn build(pair: Pair<Rule>) -> Result<Self, AlthreadError> {
        Self::from_str(pair.as_str())
    }

    pub fn from_str(a: &str) -> Result<Self, AlthreadError> {
        match a {
            "int" => Ok(Self::Int),
            "float" => Ok(Self::Float),
            "bool" => Ok(Self::Bool),
            "string" => Ok(Self::String),
            "void" => Ok(Self::Void),
            // TODO: Add error handling
            _ => unimplemented!(),
        }
    }
}

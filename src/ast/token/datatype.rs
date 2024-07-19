use std::fmt;

use pest::iterators::Pair;

use crate::{error::AlthreadError, parser::Rule};

#[derive(Debug, PartialEq, Clone)]
pub enum DataType {
    Int,
    Float,
    Bool,
    String,
    Void,
}

impl DataType {
    pub fn new() -> Self {
        Self::Void
    }
}

impl fmt::Display for DataType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            DataType::Int => write!(f, "int"),
            DataType::Float => write!(f, "float"),
            DataType::Bool => write!(f, "bool"),
            DataType::String => write!(f, "string"),
            DataType::Void => write!(f, "void"),
        }
    }
}

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

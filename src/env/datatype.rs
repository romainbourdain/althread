use std::fmt;

use crate::parser::Rule;

#[derive(Debug, PartialEq, Clone)]
pub enum DataType {
    Void,
    Bool,
    Int,
    Float,
    String,
}

impl DataType {
    pub fn from_str(value: &str) -> Self {
        match value {
            "bool" => Self::Bool,
            "int" => Self::Int,
            "float" => Self::Float,
            "string" => Self::String,
            _ => Self::Void,
        }
    }

    pub fn from_rule(rule: Rule) -> Self {
        match rule {
            Rule::BOOLEAN => Self::Bool,
            Rule::INTEGER => Self::Int,
            Rule::FLOAT => Self::Float,
            Rule::STRING => Self::String,
            _ => Self::Void,
        }
    }

    pub fn as_str(&self) -> &str {
        match self {
            DataType::Void => "void",
            DataType::Bool => "bool",
            DataType::Int => "int",
            DataType::Float => "float",
            DataType::String => "string",
        }
    }

    pub fn is_numeric(&self) -> bool {
        matches!(self, DataType::Int | DataType::Float)
    }
}

impl fmt::Display for DataType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

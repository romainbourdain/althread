use std::{collections::HashMap, fmt};

use crate::parser::Rule;

pub type SymbolTable = HashMap<String, Symbol>;

#[derive(Debug)]
pub struct Symbol {
    pub datatype: DataType,
    pub mutable: bool,
    pub value: Option<SymbolValue>,
}

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

    pub fn is_numeric(&self) -> bool {
        matches!(self, DataType::Int | DataType::Float)
    }
}

impl fmt::Display for DataType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let value = match self {
            Self::Void => "void",
            Self::Bool => "bool",
            Self::Int => "int",
            Self::Float => "float",
            Self::String => "string",
        };
        write!(f, "{}", value)
    }
}

#[derive(Debug)]
pub enum SymbolValue {
    Null,
    Bool(bool),
    Int(i64),
    Float(f64),
    String(String),
}

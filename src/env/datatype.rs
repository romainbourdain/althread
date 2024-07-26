use std::fmt;

use super::value::Value;

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

    pub fn from_value(val: &Value) -> Self {
        match val {
            Value::Null => Self::Void,
            Value::Bool(_) => Self::Bool,
            Value::Int(_) => Self::Int,
            Value::Float(_) => Self::Float,
            Value::String(_) => Self::String,
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

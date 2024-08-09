use core::fmt;

use pest::iterators::Pair;

use crate::{error::AlthreadResult, no_rule, parser::Rule};

use super::token::FromPair;

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

    /*     pub fn from_value(val: &Value) -> Self {
        match val {
            Value::Null => Self::Void,
            Value::Bool(_) => Self::Bool,
            Value::Int(_) => Self::Int,
            Value::Float(_) => Self::Float,
            Value::String(_) => Self::String,
        }
    } */

    pub fn as_str(&self) -> &str {
        match self {
            DataType::Void => "void",
            DataType::Bool => "bool",
            DataType::Int => "int",
            DataType::Float => "float",
            DataType::String => "string",
        }
    }

    pub fn can_unary(&self) -> Result<(), String> {
        match self {
            Self::Int | Self::Float => Ok(()),
            _ => Err(format!("Cannot increment {}", self)),
        }
    }

    pub fn can_add(&self, other: &DataType) -> Result<(), String> {
        match (self, other) {
            (Self::Int, Self::Int) | (Self::Float, Self::Float) | (Self::String, Self::String) => {
                Ok(())
            }
            (i, j) => Err(format!("Cannot add {} and {}", i, j)),
        }
    }

    pub fn can_arithmetic(&self, other: &DataType) -> Result<(), String> {
        match (self, other) {
            (Self::Int, Self::Int) | (Self::Float, Self::Float) => Ok(()),
            (i, j) => Err(format!("Cannot subtract {} and {}", i, j)),
        }
    }

    pub fn can_compare(&self, other: &DataType) -> Result<(), String> {
        match (self, other) {
            (Self::Int, Self::Int) | (Self::Float, Self::Float) => Ok(()),
            (i, j) => Err(format!("No modulo between {} and {}", i, j)),
        }
    }

    pub fn can_order(&self, other: &DataType) -> Result<(), String> {
        match (self, other) {
            (Self::Int, Self::Int) | (Self::Float, Self::Float) => Ok(()),
            (i, j) => Err(format!("Cannot order {} and {}", i, j)),
        }
    }

    pub fn can_logical(&self, other: &DataType) -> Result<(), String> {
        match (self, other) {
            (Self::Bool, Self::Bool) => Ok(()),
            (i, j) => Err(format!(
                "Cannot perform logical AND between {} and {}",
                i, j
            )),
        }
    }

    pub fn can_not(&self) -> Result<(), String> {
        match self {
            Self::Bool => Ok(()),
            i => Err(format!("Cannot perform logical NOT on {}", i)),
        }
    }

    pub fn can_neg(&self) -> Result<(), String> {
        match self {
            Self::Int | Self::Float => Ok(()),
            i => Err(format!("Cannot negate {}", i)),
        }
    }
}

impl FromPair for DataType {
    fn from_pair(pair: Pair<Rule>) -> AlthreadResult<Self> {
        match pair.as_str() {
            "bool" => Ok(Self::Bool),
            "int" => Ok(Self::Int),
            "float" => Ok(Self::Float),
            "string" => Ok(Self::String),
            "void" => Ok(Self::Void),
            _ => Err(no_rule!(pair)),
        }
    }
}

impl fmt::Display for DataType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

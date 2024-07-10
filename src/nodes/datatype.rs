use core::fmt;

use crate::error::AlthreadError;

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

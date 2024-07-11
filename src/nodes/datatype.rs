use core::fmt;

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

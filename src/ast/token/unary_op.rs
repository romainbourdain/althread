use std::fmt;

#[derive(Debug, Clone)]
pub enum UnOp {
    Not,
    Neg,
}

impl fmt::Display for UnOp {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use UnOp::*;
        let op = match self {
            Not => "!",
            Neg => "-",
        };
        write!(f, "{}", op)
    }
}

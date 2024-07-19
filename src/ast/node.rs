use super::token::{assign_binary_op::AssignBinaryOp, assign_unary_op::AssignUnaryOp};

#[derive(Debug)]
pub struct AstNode {
    pub line: usize,
    pub column: usize,
    pub kind: AstNodeKind,
}

impl AstNode {
    pub fn new(line: usize, column: usize, kind: AstNodeKind) -> Self {
        Self { line, column, kind }
    }
}

#[derive(Debug)]
pub enum AstNodeKind {
    AssignBinaryOp(AssignBinaryOp),
    AssignUnaryOp(AssignUnaryOp),
}

impl AstNodeKind {
    pub fn get(&self) -> &dyn std::fmt::Debug {
        match self {
            AstNodeKind::AssignBinaryOp(op) => op,
            AstNodeKind::AssignUnaryOp(op) => op,
        }
    }
}

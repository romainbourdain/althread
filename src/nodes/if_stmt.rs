use super::{block::Block, expr::Expr};

#[derive(Debug)]
pub struct IfStmt {
    pub condition: Expr,
    pub block: Block,
    pub else_block: Option<Block>,
    pub line: usize,
    pub column: usize,
}

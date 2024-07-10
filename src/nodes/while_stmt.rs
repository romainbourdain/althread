use super::{block::Block, expr::Expr};

#[derive(Debug)]
pub struct WhileStmt {
    pub condition: Expr,
    pub block: Block,
    pub line: usize,
    pub column: usize,
}

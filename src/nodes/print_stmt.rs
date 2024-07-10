use super::expr::Expr;

#[derive(Debug)]
pub struct PrintStmt {
    pub expr: Expr,
    pub line: usize,
    pub column: usize,
}

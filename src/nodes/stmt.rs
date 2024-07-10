use super::{
    assign::Assign, block::Block, decl::Decl, expr::Expr, if_stmt::IfStmt, print_stmt::PrintStmt,
    while_stmt::WhileStmt,
};

#[derive(Debug)]
pub enum Stmt {
    Expr(Expr),
    Decl(Decl),
    Print(PrintStmt),
    Block(Block),
    Assign(Assign),
    IfStmt(IfStmt),
    WhileStmt(WhileStmt),
}

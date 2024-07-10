use super::stmt::Stmt;

#[derive(Debug)]
pub struct Block {
    pub stmts: Vec<Stmt>,
    pub line: usize,
    pub column: usize,
}

impl Block {
    pub fn new(line: usize, column: usize) -> Self {
        Self {
            stmts: Vec::new(),
            line,
            column,
        }
    }
}

pub mod assign;
pub mod block;
pub mod datatype;
pub mod decl;
pub mod expr;
pub mod if_stmt;
pub mod print_stmt;
pub mod stmt;
pub mod while_stmt;

use block::Block;

#[derive(Debug)]
pub struct Ast {
    pub main_block: Option<Block>,
    pub shared_block: Option<Block>,
    pub line: usize,
    pub column: usize,
}

impl Ast {
    pub fn new(line: usize, column: usize) -> Self {
        Self {
            main_block: None,
            shared_block: None,
            line,
            column,
        }
    }
}

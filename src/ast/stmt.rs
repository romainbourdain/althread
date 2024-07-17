use pest::iterators::Pair;

use crate::{env::Environment, error::AlthreadError, parser::Rule};

use super::{
    assign::Assign, block::Block, decl::Decl, expr::Expr, if_stmt::IfStmt, print_stmt::PrintStmt,
    run_stmt::RunStmt, while_stmt::WhileStmt,
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
    Run(RunStmt),
}

impl Stmt {
    pub fn build(pair: Pair<Rule>, env: &mut Environment) -> Result<Self, AlthreadError> {
        match pair.as_rule() {
            Rule::decl => Ok(Self::Decl(Decl::build(pair, env)?)),
            Rule::expr => Ok(Self::Expr(Expr::build(pair, env)?)),
            Rule::print_stmt => Ok(Self::Print(PrintStmt::build(pair, env)?)),
            Rule::block => Ok(Self::Block(Block::parse_and_push(pair, env)?)),
            Rule::assignment => Ok(Self::Assign(Assign::build(pair, env)?)),
            Rule::if_stmt => Ok(Self::IfStmt(IfStmt::build(pair, env)?)),
            Rule::while_stmt => Ok(Self::WhileStmt(WhileStmt::build(pair, env)?)),
            Rule::run_stmt => Ok(Self::Run(RunStmt::parse(pair, env)?)),
            _ => unreachable!(),
        }
    }
}

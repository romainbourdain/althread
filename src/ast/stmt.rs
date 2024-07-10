use pest::iterators::Pair;

use crate::{env::Environment, error::AlthreadError, parser::Rule};

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
            _ => unreachable!(),
        }
    }

    pub fn eval(&self, env: &mut Environment) -> Result<(), AlthreadError> {
        use Stmt::*;
        match self {
            Expr(expr) => {
                expr.eval(env)?;
            }
            Decl(decl) => decl.eval(env)?,
            Print(print_stmt) => print_stmt.eval(env)?,
            Block(block) => block.eval(env)?,
            Assign(assign) => assign.eval(env)?,
            IfStmt(if_stmt) => if_stmt.eval(env)?,
            WhileStmt(while_stmt) => while_stmt.eval(env)?,
        };
        Ok(())
    }
}

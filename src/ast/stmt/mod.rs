pub mod assign;
pub mod decl;
pub mod if_stmt;
pub mod print_stmt;
pub mod run_stmt;
pub mod while_stmt;

use std::io::Write;

use assign::Assign;
use decl::Decl;
use if_stmt::IfStmt;
use pest::iterators::Pair;
use print_stmt::PrintStmt;
use run_stmt::RunStmt;
use while_stmt::WhileStmt;

use crate::{env::Environment, error::AlthreadError, parser::Rule};

use super::{block::Block, expr::Expr};

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

    pub fn eval<W>(&self, env: &mut Environment, output: &mut W) -> Result<(), AlthreadError>
    where
        W: Write,
    {
        use Stmt::*;
        match self {
            Assign(assign) => assign.eval(env)?,
            Expr(expr) => {
                expr.eval(env)?;
            }
            Decl(decl) => decl.eval(env)?,
            Print(print_stmt) => print_stmt.eval(env, output)?,
            Block(block) => block.eval_and_push(env, output)?,
            IfStmt(if_stmt) => if_stmt.eval(env, output)?,
            WhileStmt(while_stmt) => while_stmt.eval(env, output)?,
            Run(_) => unimplemented!(),
        };
        Ok(())
    }
}

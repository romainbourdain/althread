use pest::iterators::Pair;

use crate::{env::Environment, error::AlthreadError, parser::Rule};

use super::{
    assign::Assign,
    block::{Block, IfBlock, WhileBlock},
    decl::Decl,
    expr::Expr,
};

#[derive(Debug)]
pub enum Stmt {
    Expr(Expr),
    Decl(Decl),
    Print(Expr),
    Block(Block),
    Assign(Assign),
    IfStmt(IfBlock),
    WhileStmt(WhileBlock),
}

impl Stmt {
    pub fn build(pair: Pair<Rule>, env: &mut Environment) -> Result<Self, AlthreadError> {
        match pair.as_rule() {
            Rule::decl => Ok(Self::Decl(Decl::build(pair, env)?)),
            Rule::expr => Ok(Self::Expr(Expr::build(pair, env)?)),
            Rule::print_stmt => Ok(Self::Print(Expr::build(
                pair.into_inner().next().unwrap(),
                env,
            )?)),
            Rule::block => Ok(Self::Block(Block::parse_and_push(pair, env)?)),
            Rule::assignment => Ok(Self::Assign(Assign::build(pair, env)?)),
            Rule::if_stmt => Ok(Self::IfStmt(IfBlock::build(pair, env)?)),
            Rule::while_stmt => Ok(Self::WhileStmt(WhileBlock::build(pair, env)?)),
            _ => unreachable!(),
        }
    }
}

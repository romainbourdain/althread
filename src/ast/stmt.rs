use pest::iterators::Pair;

use crate::{env::Environment, error::AlthreadError, parser::Rule};

use super::{
    assign::Assign,
    block::{parse_block, Block, IfBlock, WhileBlock},
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
            Rule::decl => Ok(Self::Decl(Decl::build(pair.into_inner(), env)?)),
            Rule::expr => Ok(Self::Expr(Expr::build(pair.into_inner(), env)?)),
            Rule::print_stmt => Ok(Self::Print(Expr::build(
                pair.into_inner().next().unwrap().into_inner(),
                env,
            )?)),
            Rule::block => Ok(Self::Block(parse_block(pair.into_inner(), env)?)),
            Rule::assignment => Ok(Self::Assign(Assign::build(pair.into_inner(), env)?)),
            Rule::if_stmt => Ok(Self::IfStmt(IfBlock::build(pair.into_inner(), env)?)),
            Rule::while_stmt => Ok(Self::WhileStmt(WhileBlock::build(pair.into_inner(), env)?)),
            _ => unreachable!(),
        }
    }
}

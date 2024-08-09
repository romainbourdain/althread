pub mod assign;
pub mod decl;
pub mod expr;
pub mod if_block;
pub mod print;
pub mod run;
pub mod scope;
pub mod while_block;

use assign::Assign;
use decl::Decl;
use expr::Expr;
use if_block::IfBlock;
use pest::iterators::Pair;
use print::Print;
use run::Run;
use scope::Scope;
use while_block::WhileBlock;

use crate::{error::AlthreadResult, no_rule, parser::Rule};

#[derive(Debug)]
pub enum Node {
    Assign(Assign),
    Decl(Decl),
    Expr(Expr),
    Run(Run),
    Print(Print),
    If(IfBlock),
    While(WhileBlock),
    Scope(Scope),
}

impl Node {
    pub fn build(pair: Pair<Rule>) -> AlthreadResult<Self> {
        match pair.as_rule() {
            Rule::assignment => Ok(Self::Assign(Assign::build(pair)?)),
            Rule::decl => Ok(Self::Decl(Decl::build(pair)?)),
            Rule::expr => Ok(Self::Expr(Expr::build(pair)?)),
            Rule::print_stmt => Ok(Self::Print(Print::build(pair)?)),
            Rule::run_stmt => Ok(Self::Run(Run::build(pair)?)),
            Rule::if_stmt => Ok(Self::If(IfBlock::build(pair)?)),
            Rule::while_stmt => Ok(Self::While(WhileBlock::build(pair)?)),
            Rule::scope => Ok(Self::Scope(Scope::build(pair)?)),
            _ => Err(no_rule!(pair)),
        }
    }

    pub fn is_atomic(&self) -> bool {
        match self {
            Self::Assign(_) | Self::Decl(_) | Self::Expr(_) | Self::Print(_) | Self::Run(_) => true,
            _ => false,
        }
    }
}

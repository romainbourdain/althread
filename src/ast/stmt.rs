use pest::iterators::Pair;

use crate::{env::Environment, error::AlthreadError, parser::Rule};

use super::{decl::Decl, expr::Expr};

#[derive(Debug)]
pub enum Stmt {
    Expr(Expr),
    Decl(Decl),
}

impl Stmt {
    pub fn build(pair: Pair<Rule>, env: &mut Environment) -> Result<Self, AlthreadError> {
        match pair.as_rule() {
            Rule::decl => Ok(Self::Decl(Decl::build(pair.into_inner(), env)?)),
            Rule::expr => Ok(Self::Expr(Expr::build(pair.into_inner(), env)?)),
            _ => unreachable!(),
        }
    }
}

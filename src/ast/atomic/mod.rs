pub mod assign;
pub mod call;
pub mod decl;
pub mod expr;

use assign::consume_assign;
use call::consume_call;
use decl::consume_decl;
use expr::consume_expr;
use pest::iterators::Pair;

use crate::{env::Environment, error::AlthreadResult, parser::Rule};

#[derive(Debug)]
pub struct Atomic<'a> {
    pub kind: AtomicKind,
    pub pair: Pair<'a, Rule>,
}

#[derive(Debug)]
pub enum AtomicKind {
    Assignment,
    Decl,
    Expr,
    Print,
    Run,
}

impl Atomic<'_> {
    pub fn new<'a>(pair: Pair<'a, Rule>) -> Atomic<'a> {
        Atomic {
            kind: match pair.as_rule() {
                Rule::assignment => AtomicKind::Assignment,
                Rule::decl => AtomicKind::Decl,
                Rule::expr => AtomicKind::Expr,
                Rule::print_stmt => AtomicKind::Print,
                Rule::run_stmt => AtomicKind::Run,
                _ => panic!("Invalid atomic rule"),
            },
            pair,
        }
    }

    pub fn consume(&self, env: &mut Environment) -> AlthreadResult<()> {
        let pair = self.pair.clone();
        println!("{:?}", pair.as_str());
        match self.kind {
            AtomicKind::Expr => {
                consume_expr(pair, env)?;
            }
            AtomicKind::Print => {
                consume_call(pair, env)?;
            }
            AtomicKind::Decl => {
                consume_decl(pair, env)?;
            }
            AtomicKind::Assignment => {
                consume_assign(pair, env)?;
            }
            AtomicKind::Run => {
                unimplemented!();
            }
        }
        Ok(())
    }
}

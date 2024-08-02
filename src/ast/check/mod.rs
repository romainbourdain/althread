pub mod assign;
pub mod call;
pub mod decl;
pub mod expr;

use assign::check_assign;
use call::check_call;
use decl::check_decl;
use expr::check_expr;

use crate::{env::Environment, error::AlthreadResult, no_rule, parser::Rule};

use super::{
    node::{Atomic, Node},
    Ast, Brick,
};

impl<'a> Ast<'a> {
    pub fn check(&self, env: &mut Environment) -> AlthreadResult<()> {
        for (_, brick) in &self.process_bricks {
            brick.check(env)?;
        }
        Ok(())
    }
}

impl<'a> Brick<'a> {
    pub fn check(&self, env: &mut Environment) -> AlthreadResult<()> {
        env.push_table();
        for node in &self.nodes {
            node.check(env)?;
        }
        env.pop_table();
        Ok(())
    }
}

impl<'a> Node<'a> {
    pub fn check(&self, env: &mut Environment) -> AlthreadResult<()> {
        match self {
            Node::Atomic(atomic) => atomic.check(env)?,
            Node::Block(block) => {
                unimplemented!()
            }
        }
        Ok(())
    }
}

impl<'a> Atomic<'a> {
    pub fn check(&self, env: &mut Environment) -> AlthreadResult<()> {
        match self.pair.as_rule() {
            Rule::assignment => check_assign(self.pair.clone(), env)?,
            Rule::decl => check_decl(self.pair.clone(), env)?,

            Rule::expr => {
                check_expr(self.pair.clone(), env)?;
            }
            Rule::print_stmt => check_call(self.pair.clone(), env)?,
            Rule::run_stmt => {
                unimplemented!()
            }
            _ => return Err(no_rule!(self.pair)),
        }
        Ok(())
    }
}

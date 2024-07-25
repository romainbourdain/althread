pub mod decl;
pub mod expr;

use decl::check_decl;
use expr::check_expr;
use pest::iterators::Pairs;

use crate::{env::Environment, error::AlthreadResult, no_rule, parser::Rule};

use super::Ast;

impl<'a> Ast<'a> {
    pub fn check(&self, env: &mut Environment) -> AlthreadResult<()> {
        for (_, pairs) in &self.process_bricks {
            Self::check_pair(pairs.clone(), env)?;
        }
        Ok(())
    }

    fn check_pair(pairs: Pairs<'a, Rule>, env: &mut Environment) -> AlthreadResult<()> {
        for pair in pairs {
            match pair.as_rule() {
                Rule::decl => check_decl(pair, env)?,
                Rule::expr => {
                    check_expr(pair)?;
                }
                _ => return Err(no_rule!(pair)),
            }
        }

        Ok(())
    }
}

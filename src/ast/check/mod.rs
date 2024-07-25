pub mod call;
pub mod decl;
pub mod expr;

use call::check_call;
use decl::check_decl;
use expr::check_expr;
use pest::iterators::Pairs;

use crate::{env::Environment, error::AlthreadResult, no_rule, parser::Rule};

use super::Ast;

impl<'a> Ast<'a> {
    pub fn check(&self, env: &mut Environment) -> AlthreadResult<()> {
        for (_, pairs) in &self.process_bricks {
            env.push_table();
            Self::check_pair(pairs.clone(), env)?;
            println!("{:#?}", env);
            env.pop_table();
        }
        Ok(())
    }

    fn check_pair(pairs: Pairs<'a, Rule>, env: &mut Environment) -> AlthreadResult<()> {
        for pair in pairs {
            match pair.as_rule() {
                Rule::expr => {
                    check_expr(pair, env)?;
                }
                Rule::print_stmt => check_call(pair, env)?,
                Rule::decl => check_decl(pair, env)?,
                Rule::assignment
                | Rule::run_stmt
                | Rule::if_stmt
                | Rule::while_stmt
                | Rule::scope => {
                    unimplemented!()
                }
                _ => return Err(no_rule!(pair)),
            }
        }

        Ok(())
    }
}

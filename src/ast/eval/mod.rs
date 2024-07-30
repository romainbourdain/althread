pub mod assign;
pub mod call;
pub mod decl;
pub mod expr;

use assign::eval_assign;
use call::eval_call;
use decl::eval_decl;
use expr::eval_expr;
use pest::iterators::Pairs;

use crate::{debug::Debug, env::Environment, error::AlthreadResult, no_rule, parser::Rule};

use super::Ast;

impl<'a> Ast<'a> {
    pub fn eval(&self, env: &mut Environment) -> AlthreadResult<()> {
        let mut debug = Debug::new();
        for (_, pairs) in &self.process_bricks {
            env.push_table();
            eval_pairs(pairs.clone(), env, &mut debug)?;
            env.pop_table();
        }

        Ok(())
    }
}

fn eval_pairs<'a>(
    pairs: Pairs<'a, Rule>,
    env: &mut Environment,
    debug: &mut Debug,
) -> AlthreadResult<()> {
    for pair in pairs {
        debug.push(pair.as_str().to_string());

        match pair.as_rule() {
            Rule::expr => {
                eval_expr(pair, env)?;
            }
            Rule::print_stmt => eval_call(pair, env)?,
            Rule::decl => eval_decl(pair, env)?,
            Rule::assignment => eval_assign(pair, env)?,
            Rule::run_stmt | Rule::if_stmt | Rule::while_stmt | Rule::scope => {
                unimplemented!()
            }
            _ => return Err(no_rule!(pair)),
        }

        debug.prompt_user(env);
    }

    Ok(())
}

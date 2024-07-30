pub mod assign;
pub mod call;
pub mod decl;
pub mod expr;

use assign::eval_assign;
use call::eval_call;
use decl::eval_decl;
use expr::eval_expr;
use pest::iterators::Pairs;

use crate::{
    args::Config, debug::Debug, env::Environment, error::AlthreadResult, no_rule, parser::Rule,
};

use super::Ast;

impl<'a> Ast<'a> {
    pub fn eval(&self, env: &mut Environment, config: &Config) -> AlthreadResult<()> {
        for (_, pairs) in &self.process_bricks {
            env.push_table();
            eval_pairs(pairs.clone(), env, config)?;
            env.pop_table();
        }

        Ok(())
    }
}

fn eval_pairs<'a>(
    mut pairs: Pairs<'a, Rule>,
    env: &mut Environment,
    config: &Config,
) -> AlthreadResult<()> {
    loop {
        if !consume_pair(&mut pairs, env, config)? {
            break;
        };
    }

    Ok(())
}

fn consume_pair(
    pairs: &mut Pairs<Rule>,
    env: &mut Environment,
    config: &Config,
) -> AlthreadResult<bool> {
    match pairs.next() {
        None => Ok(false),
        Some(pair) => {
            match pair.as_rule() {
                Rule::expr => {
                    eval_expr(pair, env)?;
                }
                Rule::print_stmt => eval_call(pair, env)?,
                Rule::decl => eval_decl(pair, env)?,
                Rule::assignment => eval_assign(pair, env)?,
                Rule::scope | Rule::run_stmt | Rule::while_stmt | Rule::if_stmt => {
                    unimplemented!()
                }
                _ => return Err(no_rule!(pair)),
            }
            Ok(true)
        }
    }
}

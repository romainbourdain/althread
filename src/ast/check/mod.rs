pub mod call;

use call::check_call;
use pest::iterators::Pairs;

use crate::{env::Environment, error::AlthreadResult, no_rule, parser::Rule};

use super::{
    eval::{assign::eval_assign, decl::eval_decl, expr::eval_expr},
    Ast,
};

impl<'a> Ast<'a> {
    pub fn check(&self, env: &mut Environment) -> AlthreadResult<()> {
        for (_, pairs) in &self.process_bricks {
            env.push_table();
            check_pairs(pairs.clone(), env)?;
            env.pop_table();
        }
        Ok(())
    }
}

fn check_pairs<'a>(pairs: Pairs<'a, Rule>, env: &mut Environment) -> AlthreadResult<()> {
    for pair in pairs {
        match pair.as_rule() {
            Rule::expr => {
                eval_expr(pair, env)?;
            }
            Rule::print_stmt => check_call(pair, env)?,
            Rule::decl => eval_decl(pair, env)?,
            Rule::assignment => eval_assign(pair, env)?,
            Rule::run_stmt | Rule::if_stmt | Rule::while_stmt | Rule::scope => {
                unimplemented!()
            }
            _ => return Err(no_rule!(pair)),
        }
    }

    Ok(())
}

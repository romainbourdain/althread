use crate::{
    args::Config,
    ast::node::{Block, Node},
    env::Environment,
    error::AlthreadResult,
    no_rule,
    parser::Rule,
};

use super::expr::eval_expr;

pub fn eval_scope<'a>(
    block: &mut Block,
    env: &mut Environment,
    config: &Config,
) -> AlthreadResult<bool> {
    if block.children.is_empty() {
        return Ok(false);
    }

    if block.current == 0 {
        env.push_table();
    }

    if !block.children[block.current].consume(env, config)? {
        block.current += 1;
    }

    if block.current >= block.children.len() {
        env.pop_table();
        return Ok(false);
    }

    Ok(true)
}

pub fn eval_if<'a>(
    block: &mut Block,
    env: &mut Environment,
    config: &Config,
) -> AlthreadResult<bool> {
    if block.current == 0 {
        block.current = if evaluate_condition(&block.children[0], env)? {
            1
        } else {
            2
        };
        Ok(true)
    } else if block.current < block.children.len() {
        Ok(block.children[block.current].consume(env, config)?)
    } else {
        Ok(true)
    }
}

pub fn evaluate_condition(node: &Node, env: &mut Environment) -> AlthreadResult<bool> {
    Ok(match node {
        Node::Atomic(atomic) => match atomic.pair.as_rule() {
            Rule::expr => {
                println!("{:?}", atomic.pair.as_str());
                let val = eval_expr(atomic.pair.clone(), env)?;
                val.is_true()
            }
            _ => return Err(no_rule!(atomic.pair)),
        },
        _ => unreachable!(),
    })
}

pub fn eval_while<'a>(
    block: &mut Block,
    env: &mut Environment,
    config: &Config,
) -> AlthreadResult<bool> {
    if block.current == 0 {
        block.reset();
        if evaluate_condition(&block.children[0], env)? {
            block.current = 1;
        } else {
            return Ok(false);
        };
    } else if block.current < block.children.len() {
        if !block.children[block.current].consume(env, config)? {
            block.current = 0;
            return Ok(true);
        }
    }

    Ok(true)
}

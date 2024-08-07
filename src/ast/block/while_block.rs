use crate::{
    args::Config, ast::atomic::expr::consume_condition, env::Environment, error::AlthreadResult,
};

use super::Block;

pub fn consume_while<'a>(
    block: &mut Block,
    env: &mut Environment,
    config: &Config,
) -> AlthreadResult<bool> {
    if block.current == 0 {
        block.reset();
        if consume_condition(&block.children[0], env)? {
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

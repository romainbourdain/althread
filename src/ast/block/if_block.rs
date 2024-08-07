use crate::{
    args::Config, ast::atomic::expr::consume_condition, env::Environment, error::AlthreadResult,
};

use super::Block;

pub fn consume_if(
    block: &mut Block,
    env: &mut Environment,
    config: &Config,
) -> AlthreadResult<bool> {
    if block.current == 0 {
        block.current = if consume_condition(&block.children[0], env)? {
            1
        } else {
            2
        };
        Ok(true)
    } else if block.current < block.children.len() {
        Ok(block.children[block.current].consume(env, config)?)
    } else {
        Ok(false)
    }
}

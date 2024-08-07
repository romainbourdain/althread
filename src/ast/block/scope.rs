use crate::{args::Config, env::Environment, error::AlthreadResult};

use super::Block;

pub fn consume_scope<'a>(
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

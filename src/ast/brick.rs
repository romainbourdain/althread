use pest::iterators::Pairs;

use crate::{args::Config, ast::node::Node, env::Environment, error::AlthreadResult, parser::Rule};

#[derive(Debug)]
pub struct Brick<'a> {
    pub nodes: Vec<Node<'a>>,
    pub current: usize,
}

impl Brick<'_> {
    pub fn build<'a>(pairs: Pairs<'a, Rule>) -> AlthreadResult<Brick<'a>> {
        let mut nodes = Vec::new();
        for pair in pairs {
            nodes.push(Node::build(pair)?);
        }
        Ok(Brick { nodes, current: 0 })
    }

    pub fn consume(&mut self, env: &mut Environment, config: &Config) -> AlthreadResult<bool> {
        if self.nodes.is_empty() {
            return Ok(false);
        }

        if self.current == 0 {
            env.push_table();
        }

        if !self.nodes[self.current].consume(env, config)? {
            self.current += 1;
        }

        if self.current >= self.nodes.len() {
            env.pop_table();
            return Ok(false);
        }

        Ok(true)
    }
}

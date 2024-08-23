use std::fmt;

use pest::iterators::Pairs;

use crate::{
    ast::{
        display::{AstDisplay, Prefix},
        node::{Node, NodeBuilder, NodeExecutor},
        statement::expression::Expression,
    },
    env::{
        node_result::{NodeResult, Suspend},
        process_env::ProcessEnv,
    },
    error::AlthreadResult,
    parser::Rule,
};

#[derive(Debug)]
pub struct WaitCall {
    pub value: Node<Expression>,
}

impl NodeBuilder for WaitCall {
    fn build(mut pairs: Pairs<Rule>) -> AlthreadResult<Self> {
        let value = Node::build(pairs.next().unwrap())?;

        Ok(Self { value })
    }
}

impl NodeExecutor for WaitCall {
    fn eval(&self, env: &mut ProcessEnv) -> AlthreadResult<NodeResult> {
        if !self.value.eval(env)?.get_return().is_true() {
            Ok(NodeResult::Suspend(Suspend {
                process_id: env.id,
                condition: self.value.clone(),
            }))
        } else {
            Ok(NodeResult::null())
        }
    }
}

impl AstDisplay for WaitCall {
    fn ast_fmt(&self, f: &mut fmt::Formatter, prefix: &Prefix) -> fmt::Result {
        writeln!(f, "{prefix}print")?;
        self.value.ast_fmt(f, &prefix.add_leaf())?;

        Ok(())
    }
}

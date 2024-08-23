use std::fmt;

use pest::iterators::Pairs;

use crate::{
    ast::{
        display::{AstDisplay, Prefix},
        node::{Node, NodeBuilder, NodeExecutor},
        statement::expression::Expression,
    },
    env::{node_result::NodeResult, process_env::ProcessEnv},
    error::AlthreadResult,
    parser::Rule,
};

#[derive(Debug)]
pub struct PrintCall {
    pub value: Node<Expression>,
}

impl NodeBuilder for PrintCall {
    fn build(mut pairs: Pairs<Rule>) -> AlthreadResult<Self> {
        let value = Node::build(pairs.next().unwrap())?;

        Ok(Self { value })
    }
}

impl NodeExecutor for PrintCall {
    fn eval(&self, env: &mut ProcessEnv) -> AlthreadResult<Option<NodeResult>> {
        if let Some(value) = self.value.eval(env)? {
            println!("{}", value.get_literal());
        }

        Ok(Some(NodeResult::Null))
    }
}

impl AstDisplay for PrintCall {
    fn ast_fmt(&self, f: &mut fmt::Formatter, prefix: &Prefix) -> fmt::Result {
        writeln!(f, "{prefix}print")?;
        self.value.ast_fmt(f, &prefix.add_leaf())?;

        Ok(())
    }
}

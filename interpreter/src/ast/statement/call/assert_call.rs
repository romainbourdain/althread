use std::fmt;

use pest::iterators::Pairs;

use crate::{
    ast::{
        display::{AstDisplay, Prefix},
        node::{Node, NodeBuilder, NodeExecutor},
        statement::expression::Expression,
    },
    env::{node_result::NodeResult, process_env::ProcessEnv},
    error::{AlthreadError, AlthreadResult, ErrorType},
    parser::Rule,
};

#[derive(Debug)]
pub struct AssertCall {
    pub value: Node<Expression>,
}

impl NodeBuilder for AssertCall {
    fn build(mut pairs: Pairs<Rule>) -> AlthreadResult<Self> {
        let value = Node::build(pairs.next().unwrap())?;

        Ok(Self { value })
    }
}

impl NodeExecutor for AssertCall {
    fn eval(&self, env: &mut ProcessEnv) -> AlthreadResult<NodeResult> {
        let value = self.value.eval(env)?;
        if !value.get_return().is_true() {
            return Err(AlthreadError::new(
                ErrorType::AssertionFailed,
                self.value.line,
                self.value.column,
                format!("Condition is false"),
            ));
        }

        Ok(NodeResult::null())
    }
}

impl AstDisplay for AssertCall {
    fn ast_fmt(&self, f: &mut fmt::Formatter, prefix: &Prefix) -> fmt::Result {
        writeln!(f, "{prefix}print")?;
        self.value.ast_fmt(f, &prefix.add_leaf())?;

        Ok(())
    }
}
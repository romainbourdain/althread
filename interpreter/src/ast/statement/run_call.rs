use std::fmt;

use pest::iterators::Pairs;

use crate::{
    ast::{
        display::{AstDisplay, Prefix},
        node::{Node, NodeBuilder, NodeExecutor},
        token::literal::Literal,
    },
    env::process_table::process_env::ProcessEnv,
    error::{AlthreadError, AlthreadResult, ErrorType},
    parser::Rule,
};

#[derive(Debug)]
pub struct RunCall {
    pub identifier: Node<String>,
}

impl NodeBuilder for RunCall {
    fn build(mut pairs: Pairs<Rule>) -> AlthreadResult<Self> {
        let pair = pairs.next().unwrap();
        let identifier = Node {
            line: pair.line_col().0,
            column: pair.line_col().1,
            value: pair.as_str().to_string(),
        };

        Ok(Self { identifier })
    }
}

impl NodeExecutor for RunCall {
    fn eval(&self, env: &mut ProcessEnv) -> AlthreadResult<Option<Literal>> {
        let running_processes = &env.running_process;

        running_processes
            .borrow_mut()
            .insert(self.identifier.value.clone(), &env.process_table)
            .map_err(|e| {
                AlthreadError::new(
                    ErrorType::VariableError,
                    self.identifier.line,
                    self.identifier.column,
                    e,
                )
            })?;

        Ok(Some(Literal::Null))
    }
}

impl AstDisplay for RunCall {
    fn ast_fmt(&self, f: &mut fmt::Formatter, prefix: &Prefix) -> fmt::Result {
        writeln!(f, "{prefix}run: {}", self.identifier)?;

        Ok(())
    }
}

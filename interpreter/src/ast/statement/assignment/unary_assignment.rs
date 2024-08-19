use std::fmt::{self};

use pest::iterators::Pairs;

use crate::{
    ast::{
        display::{AstDisplay, Prefix},
        node::{Node, NodeBuilder, NodeExecutor},
        token::{
            identifier::Identifier, literal::Literal,
            unary_assignment_operator::UnaryAssignmentOperator,
        },
    },
    env::process_env::ProcessEnv,
    error::{AlthreadError, AlthreadResult, ErrorType},
    parser::Rule,
};

#[derive(Debug)]
pub struct UnaryAssignment {
    pub identifier: Node<Identifier>,
    pub operator: Node<UnaryAssignmentOperator>,
}

impl NodeBuilder for UnaryAssignment {
    fn build(mut pairs: Pairs<Rule>) -> AlthreadResult<Self> {
        let identifier = Node::build(pairs.next().unwrap())?;
        let operator = Node::build(pairs.next().unwrap())?;

        Ok(Self {
            identifier,
            operator,
        })
    }
}

impl NodeExecutor for UnaryAssignment {
    fn eval(&self, env: &mut ProcessEnv) -> AlthreadResult<Option<Literal>> {
        let current_value: Literal = env
            .symbol_table
            .borrow()
            .get(&self.identifier.value)
            .map_err(|e| {
                AlthreadError::new(
                    ErrorType::VariableError,
                    self.identifier.line,
                    self.identifier.column,
                    e,
                )
            })?
            .value;

        let value = match self.operator.value {
            UnaryAssignmentOperator::Increment => current_value.increment(),
            UnaryAssignmentOperator::Decrement => current_value.decrement(),
        }
        .map_err(|e| {
            AlthreadError::new(
                ErrorType::VariableError,
                self.identifier.line,
                self.identifier.column,
                e,
            )
        })?;

        env.symbol_table
            .borrow_mut()
            .update(&self.identifier.value, value)
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

impl AstDisplay for UnaryAssignment {
    fn ast_fmt(&self, f: &mut fmt::Formatter, prefix: &Prefix) -> fmt::Result {
        writeln!(f, "{}unary_assign", prefix)?;

        let prefix = &prefix.add_branch();
        writeln!(f, "{}ident: {}", prefix, self.identifier)?;

        let prefix = &prefix.switch();
        writeln!(f, "{}op: {}", prefix, self.operator)?;

        Ok(())
    }
}

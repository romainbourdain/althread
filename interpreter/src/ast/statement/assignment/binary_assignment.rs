use std::fmt::{self};

use pest::iterators::Pairs;

use crate::{
    ast::{
        display::{AstDisplay, Prefix},
        node::{Node, NodeBuilder, NodeExecutor},
        statement::expression::Expression,
        token::{
            binary_assignment_operator::BinaryAssignmentOperator, identifier::Identifier,
            literal::Literal,
        },
    },
    env::process_env::ProcessEnv,
    error::{AlthreadError, AlthreadResult, ErrorType},
    parser::Rule,
};

#[derive(Debug)]
pub struct BinaryAssignment {
    pub identifier: Node<Identifier>,
    pub operator: Node<BinaryAssignmentOperator>,
    pub value: Node<Expression>,
}

impl NodeBuilder for BinaryAssignment {
    fn build(mut pairs: Pairs<Rule>) -> AlthreadResult<Self> {
        let identifier = Node::build(pairs.next().unwrap())?;
        let operator = Node::build(pairs.next().unwrap())?;
        let value = Node::build(pairs.next().unwrap())?;

        Ok(Self {
            identifier,
            operator,
            value,
        })
    }
}

impl NodeExecutor for BinaryAssignment {
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

        let value = self.value.eval(env)?.unwrap();

        let value = match self.operator.value {
            BinaryAssignmentOperator::Assign => Ok(value),
            BinaryAssignmentOperator::AddAssign => current_value.add(&value),
            BinaryAssignmentOperator::SubtractAssign => current_value.subtract(&value),
            BinaryAssignmentOperator::MultiplyAssign => current_value.multiply(&value),
            BinaryAssignmentOperator::DivideAssign => current_value.divide(&value),
            BinaryAssignmentOperator::ModuloAssign => current_value.modulo(&value),
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

impl AstDisplay for BinaryAssignment {
    fn ast_fmt(&self, f: &mut fmt::Formatter, prefix: &Prefix) -> fmt::Result {
        writeln!(f, "{}binary_assign", prefix)?;

        let prefix = prefix.add_branch();
        writeln!(f, "{}ident: {}", &prefix, self.identifier)?;
        writeln!(f, "{}op: {}", &prefix, self.operator)?;

        let prefix = prefix.switch();
        writeln!(f, "{}value:", &prefix)?;
        let prefix = prefix.add_leaf();
        self.value.ast_fmt(f, &prefix)?;
        Ok(())
    }
}

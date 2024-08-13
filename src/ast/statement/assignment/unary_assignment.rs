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
    env::Env,
    error::AlthreadResult,
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
    fn eval(&self, _env: &mut Env) -> AlthreadResult<Option<Literal>> {
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

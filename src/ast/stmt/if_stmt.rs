use pest::iterators::Pair;

use crate::{
    ast::node::{Build, Node},
    error::AlthreadResult,
    parser::Rule,
};

use super::{expr::Expr, scope::Scope};

#[derive(Debug)]
pub struct IfStmt {
    pub condition: Node<Expr>,
    pub then_block: Node<Scope>,
    pub else_block: Option<Node<Scope>>,
}

impl Build for IfStmt {
    fn build(pair: Pair<Rule>) -> AlthreadResult<Self> {
        let mut pairs = pair.into_inner();

        let condition = Node::build(pairs.next().unwrap())?;
        let then_block = Node::build(pairs.next().unwrap())?;
        let else_block = pairs.next().map(|pair| Node::build(pair)).transpose()?;

        Ok(Self {
            condition,
            then_block,
            else_block,
        })
    }
}

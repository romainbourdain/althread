use pest::iterators::Pair;

use crate::{
    ast::node::{Build, Node},
    error::AlthreadResult,
    parser::Rule,
};

use super::{expr::Expr, scope::Scope};

#[derive(Debug)]
pub struct WhileStmt {
    pub condition: Node<Expr>,
    pub then_block: Node<Scope>,
}

impl Build for WhileStmt {
    fn build(pair: Pair<Rule>) -> AlthreadResult<Self> {
        let mut pairs = pair.into_inner();

        let condition = Node::build(pairs.next().unwrap())?;
        let then_block = Node::build(pairs.next().unwrap())?;

        Ok(Self {
            condition,
            then_block,
        })
    }
}

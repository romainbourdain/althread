use std::fmt;

use pest::iterators::Pairs;

use crate::{
    ast::{
        display::{AstDisplay, Prefix},
        node::{AstNode, Node},
    },
    error::AlthreadResult,
    parser::Rule,
};

use super::{expression::Expression, scope::Scope};

#[derive(Debug)]
pub struct WhileControl {
    pub condition: Node<Expression>,
    pub then_block: Node<Scope>,
}

impl AstNode for WhileControl {
    fn build(mut pairs: Pairs<Rule>) -> AlthreadResult<Self> {
        let condition = Node::build(pairs.next().unwrap())?;
        let then_block = Node::build(pairs.next().unwrap())?;

        Ok(Self {
            condition,
            then_block,
        })
    }
}

impl AstDisplay for WhileControl {
    fn ast_fmt(&self, f: &mut fmt::Formatter, prefix: &Prefix) -> fmt::Result {
        writeln!(f, "{prefix}if_stmt")?;

        let prefix = prefix.add_branch();
        writeln!(f, "{prefix}condition")?;
        {
            let prefix = prefix.add_leaf();
            self.condition.ast_fmt(f, &prefix)?;
        }

        let prefix = prefix.switch();
        writeln!(f, "{prefix}then")?;
        {
            let prefix = prefix.add_leaf();
            self.then_block.ast_fmt(f, &prefix)?;
        }

        Ok(())
    }
}

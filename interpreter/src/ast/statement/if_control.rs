use std::fmt;

use pest::iterators::Pairs;

use crate::{
    ast::{
        display::{AstDisplay, Prefix},
        node::{Node, NodeBuilder, NodeExecutor},
        token::literal::Literal,
    },
    env::process_env::ProcessEnv,
    error::AlthreadResult,
    parser::Rule,
};

use super::{expression::Expression, scope::Scope};

#[derive(Debug)]
pub struct IfControl {
    pub condition: Node<Expression>,
    pub then_block: Node<Scope>,
    pub else_block: Option<Node<Scope>>,
}

impl NodeBuilder for IfControl {
    fn build(mut pairs: Pairs<Rule>) -> AlthreadResult<Self> {
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

impl NodeExecutor for IfControl {
    fn eval(&self, env: &mut ProcessEnv) -> AlthreadResult<Option<Literal>> {
        match env.position {
            0 => {
                let condition = self.condition.eval(env.get_child())?.unwrap();
                env.position = if condition.is_true() {
                    1
                } else if self.else_block.is_some() {
                    2
                } else {
                    return Ok(Some(Literal::Null));
                };
                Ok(None)
            }
            1 => Ok(self
                .then_block
                .eval(env.get_child())?
                .map(|_| Literal::Null)),
            2 => Ok(self
                .else_block
                .as_ref()
                .unwrap()
                .eval(env.get_child())?
                .map(|_| Literal::Null)),
            _ => unreachable!(),
        }
    }
}

impl AstDisplay for IfControl {
    fn ast_fmt(&self, f: &mut fmt::Formatter, prefix: &Prefix) -> fmt::Result {
        writeln!(f, "{prefix}if_control")?;

        let prefix = prefix.add_branch();
        writeln!(f, "{prefix}condition")?;
        {
            let prefix = prefix.add_leaf();
            self.condition.ast_fmt(f, &prefix)?;
        }
        if let Some(else_block) = &self.else_block {
            writeln!(f, "{prefix}then")?;
            {
                let prefix = prefix.add_leaf();
                self.then_block.ast_fmt(f, &prefix)?;
            }

            let prefix = prefix.switch();
            writeln!(f, "{prefix}else")?;
            {
                let prefix = prefix.add_leaf();
                else_block.ast_fmt(f, &prefix)?;
            }
        } else {
            let prefix = prefix.switch();
            writeln!(f, "{prefix}then")?;
            {
                let prefix = prefix.add_leaf();
                self.then_block.ast_fmt(f, &prefix)?;
            }
        }

        Ok(())
    }
}

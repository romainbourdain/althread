use std::fmt;

use pest::iterators::Pairs;

use crate::{
    ast::{
        display::{AstDisplay, Prefix},
        node::{Node, NodeBuilder, NodeExecutor},
        token::literal::Literal,
    },
    env::Env,
    error::AlthreadResult,
    parser::Rule,
};

use super::{expression::Expression, scope::Scope};

#[derive(Debug)]
pub struct WhileControl {
    pub condition: Node<Expression>,
    pub then_block: Node<Scope>,
}

impl NodeBuilder for WhileControl {
    fn build(mut pairs: Pairs<Rule>) -> AlthreadResult<Self> {
        let condition = Node::build(pairs.next().unwrap())?;
        let then_block = Node::build(pairs.next().unwrap())?;

        Ok(Self {
            condition,
            then_block,
        })
    }
}

impl NodeExecutor for WhileControl {
    fn eval(&self, env: &mut Env) -> AlthreadResult<Option<Literal>> {
        match env.position {
            0 => {
                if self.condition.eval(env.get_child())?.is_some() {
                    // TODO : Implement condition evaluation
                    env.consume();
                }
                Ok(None)
            }
            // TODO : Implement loop at end of scope
            1 => Ok(self.then_block.eval(env.get_child())?),
            _ => unreachable!(),
        }
    }
}

impl AstDisplay for WhileControl {
    fn ast_fmt(&self, f: &mut fmt::Formatter, prefix: &Prefix) -> fmt::Result {
        writeln!(f, "{prefix}while_control")?;

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

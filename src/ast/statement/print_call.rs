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

use super::expression::Expression;

#[derive(Debug)]
pub struct PrintCall {
    pub value: Node<Expression>,
}

impl NodeBuilder for PrintCall {
    fn build(mut pairs: Pairs<Rule>) -> AlthreadResult<Self> {
        let value = Node::build(pairs.next().unwrap())?;

        Ok(Self { value })
    }
}

impl NodeExecutor for PrintCall {
    fn eval(&self, _env: &mut Env) -> AlthreadResult<Option<Literal>> {
        println!("print");

        Ok(Some(Literal::Null))
    }
}

impl AstDisplay for PrintCall {
    fn ast_fmt(&self, f: &mut fmt::Formatter, prefix: &Prefix) -> fmt::Result {
        writeln!(f, "{prefix}print")?;
        self.value.ast_fmt(f, &prefix.add_leaf())?;

        Ok(())
    }
}

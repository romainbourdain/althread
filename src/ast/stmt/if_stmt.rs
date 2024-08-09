use std::fmt;

use pest::iterators::Pair;

use crate::{
    ast::{
        display::AstDisplay,
        node::{Build, Node},
    },
    error::AlthreadResult,
    parser::Rule,
    write_indent,
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

impl AstDisplay for IfStmt {
    fn ast_fmt(&self, f: &mut fmt::Formatter, indent_level: usize) -> fmt::Result {
        write_indent!(f, indent_level, "if_stmt")?;
        write_indent!(f, indent_level + 1, "condition:")?;
        self.condition.ast_fmt(f, indent_level + 2)?;
        write_indent!(f, indent_level + 1, "then_block:")?;
        self.then_block.ast_fmt(f, indent_level + 2)?;

        if let Some(else_block) = &self.else_block {
            write_indent!(f, indent_level + 1, "else_block:")?;
            else_block.ast_fmt(f, indent_level + 2)?;
        }

        Ok(())
    }
}

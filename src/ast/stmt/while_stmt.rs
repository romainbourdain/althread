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

impl AstDisplay for WhileStmt {
    fn ast_fmt(&self, f: &mut fmt::Formatter, indent_level: usize) -> fmt::Result {
        write_indent!(f, indent_level, "if_stmt")?;
        write_indent!(f, indent_level + 1, "condition:")?;
        self.condition.ast_fmt(f, indent_level + 2)?;
        write_indent!(f, indent_level + 1, "then_block:")?;
        self.then_block.ast_fmt(f, indent_level + 2)?;

        Ok(())
    }
}

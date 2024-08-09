pub mod assign;
pub mod decl;
pub mod expr;
pub mod if_stmt;
pub mod print_stmt;
pub mod run_stmt;
pub mod scope;
pub mod while_stmt;

use std::fmt;

use assign::Assign;
use decl::Decl;
use expr::Expr;
use if_stmt::IfStmt;
use pest::iterators::Pair;
use print_stmt::Print;
use run_stmt::RunStmt;
use scope::Scope;
use while_stmt::WhileStmt;

use crate::{error::AlthreadResult, no_rule, parser::Rule};

use super::{
    display::{AstDisplay, Prefix},
    node::{Build, Node},
};

#[derive(Debug)]
pub enum Stmt {
    Assign(Node<Assign>),
    Decl(Node<Decl>),
    Expr(Node<Expr>),
    Run(Node<RunStmt>),
    Print(Node<Print>),
    If(Node<IfStmt>),
    While(Node<WhileStmt>),
    Scope(Node<Scope>),
}

impl Build for Stmt {
    fn build(pair: Pair<Rule>) -> AlthreadResult<Self> {
        match pair.as_rule() {
            Rule::assignment => Ok(Self::Assign(Node::build(pair)?)),
            Rule::decl => Ok(Self::Decl(Node::build(pair)?)),
            Rule::expr => Ok(Self::Expr(Node::build(pair)?)),
            Rule::print_stmt => Ok(Self::Print(Node::build(pair)?)),
            Rule::run_stmt => Ok(Self::Run(Node::build(pair)?)),
            Rule::if_stmt => Ok(Self::If(Node::build(pair)?)),
            Rule::while_stmt => Ok(Self::While(Node::build(pair)?)),
            Rule::scope => Ok(Self::Scope(Node::build(pair)?)),
            _ => Err(no_rule!(pair)),
        }
    }
}

impl Stmt {
    pub fn is_atomic(&self) -> bool {
        match self {
            Self::Assign(_) | Self::Decl(_) | Self::Expr(_) | Self::Print(_) | Self::Run(_) => true,
            _ => false,
        }
    }
}

impl AstDisplay for Stmt {
    fn ast_fmt(&self, f: &mut fmt::Formatter, prefix: &Prefix) -> fmt::Result {
        match self {
            Stmt::Assign(node) => node.ast_fmt(f, prefix),
            Stmt::Decl(node) => node.ast_fmt(f, prefix),
            Stmt::Expr(node) => node.ast_fmt(f, prefix),
            Stmt::Print(node) => node.ast_fmt(f, prefix),
            Stmt::Run(node) => node.ast_fmt(f, prefix),
            Stmt::If(node) => node.ast_fmt(f, prefix),
            Stmt::While(node) => node.ast_fmt(f, prefix),
            Stmt::Scope(node) => node.ast_fmt(f, prefix),
        }
    }
}

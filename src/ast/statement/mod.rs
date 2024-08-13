pub mod assignment;
pub mod declaration;
pub mod expression;
pub mod if_control;
pub mod print_call;
pub mod run_call;
pub mod scope;
pub mod while_control;

use std::fmt;

use assignment::Assignment;
use declaration::Declaration;
use expression::Expression;
use if_control::IfControl;
use pest::iterators::Pairs;
use print_call::PrintCall;
use run_call::RunCall;
use scope::Scope;
use while_control::WhileControl;

use crate::{env::Env, error::AlthreadResult, no_rule, parser::Rule};

use super::{
    display::{AstDisplay, Prefix},
    node::{Node, NodeBuilder, NodeExecutor},
    token::literal::Literal,
};

#[derive(Debug)]
pub enum Statement {
    Assignment(Node<Assignment>),
    Declaration(Node<Declaration>),
    Expression(Node<Expression>),
    Run(Node<RunCall>),
    Print(Node<PrintCall>),
    If(Node<IfControl>),
    While(Node<WhileControl>),
    Scope(Node<Scope>),
}

impl NodeBuilder for Statement {
    fn build(mut pairs: Pairs<Rule>) -> AlthreadResult<Self> {
        let pair = pairs.next().unwrap();

        match pair.as_rule() {
            Rule::assignment => Ok(Self::Assignment(Node::build(pair)?)),
            Rule::declaration => Ok(Self::Declaration(Node::build(pair)?)),
            Rule::expression => Ok(Self::Expression(Node::build(pair)?)),
            Rule::print_call => Ok(Self::Print(Node::build(pair)?)),
            Rule::run_call => Ok(Self::Run(Node::build(pair)?)),
            Rule::if_control => Ok(Self::If(Node::build(pair)?)),
            Rule::while_control => Ok(Self::While(Node::build(pair)?)),
            Rule::scope => Ok(Self::Scope(Node::build(pair)?)),
            _ => Err(no_rule!(pair)),
        }
    }
}

impl NodeExecutor for Statement {
    fn eval(&self, env: &mut Env) -> AlthreadResult<Option<Literal>> {
        match self {
            Self::Assignment(node) => node.eval(env),
            Self::Declaration(node) => node.eval(env),
            Self::Expression(node) => node.eval(env),
            Self::Print(node) => node.eval(env),
            Self::Run(node) => node.eval(env),
            Self::If(node) => node.eval(env),
            Self::While(node) => node.eval(env),
            Self::Scope(node) => node.eval(env),
        }
    }
}

impl Statement {
    pub fn is_atomic(&self) -> bool {
        match self {
            Self::Assignment(_)
            | Self::Declaration(_)
            | Self::Expression(_)
            | Self::Print(_)
            | Self::Run(_) => true,
            _ => false,
        }
    }
}

impl AstDisplay for Statement {
    fn ast_fmt(&self, f: &mut fmt::Formatter, prefix: &Prefix) -> fmt::Result {
        match self {
            Statement::Assignment(node) => node.ast_fmt(f, prefix),
            Statement::Declaration(node) => node.ast_fmt(f, prefix),
            Statement::Expression(node) => node.ast_fmt(f, prefix),
            Statement::Print(node) => node.ast_fmt(f, prefix),
            Statement::Run(node) => node.ast_fmt(f, prefix),
            Statement::If(node) => node.ast_fmt(f, prefix),
            Statement::While(node) => node.ast_fmt(f, prefix),
            Statement::Scope(node) => node.ast_fmt(f, prefix),
        }
    }
}

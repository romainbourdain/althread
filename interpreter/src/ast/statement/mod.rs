pub mod assignment;
pub mod atomic_scope;
pub mod call;
pub mod declaration;
pub mod expression;
pub mod if_control;
pub mod scope;
pub mod while_control;

use std::fmt;

use assignment::Assignment;
use atomic_scope::AtomicScope;
use call::Call;
use declaration::Declaration;
use expression::Expression;
use if_control::IfControl;
use pest::iterators::Pairs;
use scope::Scope;
use while_control::WhileControl;

use crate::{
    env::{node_result::NodeResult, process_env::ProcessEnv},
    error::AlthreadResult,
    no_rule,
    parser::Rule,
};

use super::{
    display::{AstDisplay, Prefix},
    node::{Node, NodeBuilder, NodeExecutor},
};

#[derive(Debug)]
pub enum Statement {
    Assignment(Node<Assignment>),
    Declaration(Node<Declaration>),
    Expression(Node<Expression>),
    Call(Node<Call>),
    If(Node<IfControl>),
    While(Node<WhileControl>),
    Scope(Node<Scope>),
    Atomic(Node<AtomicScope>),
}

impl NodeBuilder for Statement {
    fn build(mut pairs: Pairs<Rule>) -> AlthreadResult<Self> {
        let pair = pairs.next().unwrap();

        match pair.as_rule() {
            Rule::assignment => Ok(Self::Assignment(Node::build(pair)?)),
            Rule::declaration => Ok(Self::Declaration(Node::build(pair)?)),
            Rule::expression => Ok(Self::Expression(Node::build(pair)?)),
            Rule::call => Ok(Self::Call(Node::build(pair)?)),
            Rule::if_control => Ok(Self::If(Node::build(pair)?)),
            Rule::while_control => Ok(Self::While(Node::build(pair)?)),
            Rule::scope => Ok(Self::Scope(Node::build(pair)?)),
            Rule::atomic_scope => Ok(Self::Atomic(Node::build(pair)?)),
            _ => Err(no_rule!(pair)),
        }
    }
}

impl NodeExecutor for Statement {
    fn eval(&self, env: &mut ProcessEnv) -> AlthreadResult<Option<NodeResult>> {
        match self {
            Self::Assignment(node) => node.eval(env),
            Self::Declaration(node) => node.eval(env),
            Self::Expression(node) => node.eval(env),
            Self::Call(node) => node.eval(env),
            Self::If(node) => node.eval(env),
            Self::While(node) => node.eval(env),
            Self::Scope(node) => node.eval(env),
            Self::Atomic(node) => node.eval(env),
        }
    }
}

impl AstDisplay for Statement {
    fn ast_fmt(&self, f: &mut fmt::Formatter, prefix: &Prefix) -> fmt::Result {
        match self {
            Statement::Assignment(node) => node.ast_fmt(f, prefix),
            Statement::Declaration(node) => node.ast_fmt(f, prefix),
            Statement::Expression(node) => node.ast_fmt(f, prefix),
            Statement::Call(node) => node.ast_fmt(f, prefix),
            Statement::If(node) => node.ast_fmt(f, prefix),
            Statement::While(node) => node.ast_fmt(f, prefix),
            Statement::Scope(node) => node.ast_fmt(f, prefix),
            Statement::Atomic(node) => node.ast_fmt(f, prefix),
        }
    }
}

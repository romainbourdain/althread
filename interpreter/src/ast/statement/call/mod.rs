pub mod assert_call;
pub mod print_call;
pub mod run_call;

use std::fmt;

use assert_call::AssertCall;
use pest::iterators::Pairs;
use print_call::PrintCall;
use run_call::RunCall;

use crate::{
    ast::{
        display::{AstDisplay, Prefix},
        node::{Node, NodeBuilder, NodeExecutor},
        token::literal::Literal,
    },
    env::process_env::ProcessEnv,
    error::AlthreadResult,
    no_rule,
    parser::Rule,
};

#[derive(Debug)]
pub enum Call {
    Run(Node<RunCall>),
    Print(Node<PrintCall>),
    Assert(Node<AssertCall>),
}

impl NodeBuilder for Call {
    fn build(mut pairs: Pairs<Rule>) -> AlthreadResult<Self> {
        let pair = pairs.next().unwrap();

        match pair.as_rule() {
            Rule::run_call => Ok(Self::Run(Node::build(pair)?)),
            Rule::print_call => Ok(Self::Print(Node::build(pair)?)),
            Rule::assert_call => Ok(Self::Assert(Node::build(pair)?)),
            _ => Err(no_rule!(pair)),
        }
    }
}

impl NodeExecutor for Call {
    fn eval(&self, env: &mut ProcessEnv) -> AlthreadResult<Option<Literal>> {
        match self {
            Self::Run(node) => node.eval(env),
            Self::Print(node) => node.eval(env),
            Self::Assert(node) => node.eval(env),
        }
    }
}

impl AstDisplay for Call {
    fn ast_fmt(&self, f: &mut fmt::Formatter, prefix: &Prefix) -> fmt::Result {
        match self {
            Self::Run(node) => node.ast_fmt(f, prefix),
            Self::Print(node) => node.ast_fmt(f, prefix),
            Self::Assert(node) => node.ast_fmt(f, prefix),
        }
    }
}

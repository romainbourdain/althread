use pest::iterators::Pair;

use crate::{error::AlthreadResult, no_rule, parser::Rule};

#[derive(Debug)]
pub enum Node<'a> {
    Atomic(Atomic<'a>),
    Block(Block<'a>),
}

impl Node<'_> {
    pub fn build(pair: Pair<Rule>) -> AlthreadResult<Node> {
        Ok(match pair.as_rule() {
            Rule::assignment | Rule::decl | Rule::expr | Rule::print_stmt | Rule::run_stmt => {
                Node::Atomic(Atomic::new(pair))
            }
            Rule::if_stmt | Rule::while_stmt | Rule::scope => {
                let mut block = Block::new(pair);
                for pair in block.pair.clone().into_inner() {
                    block.children.push(Self::build(pair)?);
                }
                Node::Block(block)
            }
            _ => return Err(no_rule!(pair)),
        })
    }
}

#[derive(Debug)]
pub struct Atomic<'a> {
    pub kind: AtomicKind,
    pub pair: Pair<'a, Rule>,
}

#[derive(Debug)]
pub enum AtomicKind {
    Assignment,
    Decl,
    Expr,
    Print,
    Run,
}

impl Atomic<'_> {
    pub fn new<'a>(pair: Pair<'a, Rule>) -> Atomic<'a> {
        Atomic {
            kind: match pair.as_rule() {
                Rule::assignment => AtomicKind::Assignment,
                Rule::decl => AtomicKind::Decl,
                Rule::expr => AtomicKind::Expr,
                Rule::print_stmt => AtomicKind::Print,
                Rule::run_stmt => AtomicKind::Run,
                _ => panic!("Invalid atomic rule"),
            },
            pair,
        }
    }
}

#[derive(Debug)]
pub struct Block<'a> {
    pub kind: BlockKind,
    pub pair: Pair<'a, Rule>,
    pub children: Vec<Node<'a>>,
    pub current: usize,
}

#[derive(Debug)]
pub enum BlockKind {
    Scope,
    If,
    While,
}

impl Block<'_> {
    pub fn new<'a>(pair: Pair<'a, Rule>) -> Block<'a> {
        Block {
            kind: match pair.as_rule() {
                Rule::scope => BlockKind::Scope,
                Rule::if_stmt => BlockKind::If,
                Rule::while_stmt => BlockKind::While,
                _ => panic!("Invalid block rule"),
            },
            pair,
            children: Vec::new(),
            current: 0,
        }
    }
}

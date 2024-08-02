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
    pub pair: Pair<'a, Rule>,
}

impl Atomic<'_> {
    pub fn new<'a>(pair: Pair<'a, Rule>) -> Atomic<'a> {
        Atomic { pair }
    }
}

#[derive(Debug)]
pub struct Block<'a> {
    pub pair: Pair<'a, Rule>,
    pub children: Vec<Node<'a>>,
}

impl Block<'_> {
    pub fn new<'a>(pair: Pair<'a, Rule>) -> Block<'a> {
        Block {
            pair,
            children: Vec::new(),
        }
    }
}

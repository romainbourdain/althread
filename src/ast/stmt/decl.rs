use pest::iterators::Pair;

use crate::{
    ast::{
        datatype::DataType,
        node::{Build, Node},
    },
    error::AlthreadResult,
    no_rule,
    parser::Rule,
};

use super::expr::{primary_expr::Identifier, Expr};

#[derive(Debug)]
pub struct Decl {
    pub keyword: Node<DeclKeyword>,
    pub identifier: Node<Identifier>,
    pub datatype: Option<Node<DataType>>,
    pub value: Option<Node<Expr>>,
}

impl Build for Decl {
    fn build(pair: Pair<Rule>) -> AlthreadResult<Self> {
        let mut pairs = pair.into_inner();

        let keyword = Node::build(pairs.next().unwrap())?;
        let identifier = Node::build(pairs.next().unwrap())?;
        let mut datatype = None;
        let mut value = None;

        for pair in pairs {
            match pair.as_rule() {
                Rule::DATATYPE => {
                    datatype = Some(Node::build(pair)?);
                }
                Rule::expr => {
                    value = Some(Node::build(pair)?);
                }
                _ => return Err(no_rule!(pair)),
            }
        }

        Ok(Self {
            keyword,
            identifier,
            datatype,
            value,
        })
    }
}

#[derive(Debug)]
pub enum DeclKeyword {
    Let,
    Const,
}

impl Build for DeclKeyword {
    fn build(pair: Pair<Rule>) -> AlthreadResult<Self> {
        match pair.as_str() {
            "let" => Ok(Self::Let),
            "const" => Ok(Self::Const),
            _ => Err(no_rule!(pair)),
        }
    }
}

use std::fmt;

use pest::iterators::Pairs;

use crate::{
    ast::{
        display::{AstDisplay, Prefix},
        node::{Node, NodeBuilder, NodeExecutor},
        token::{
            datatype::DataType, declaration_keyword::DeclarationKeyword, identifier::Identifier,
            literal::Literal,
        },
    },
    env::process_env::ProcessEnv,
    error::{AlthreadError, AlthreadResult, ErrorType},
    no_rule,
    parser::Rule,
};

use super::expression::Expression;

#[derive(Debug)]
pub struct Declaration {
    pub keyword: Node<DeclarationKeyword>,
    pub identifier: Node<Identifier>,
    pub datatype: Option<Node<DataType>>,
    pub value: Option<Node<Expression>>,
}

impl NodeBuilder for Declaration {
    fn build(mut pairs: Pairs<Rule>) -> AlthreadResult<Self> {
        let keyword = Node::build(pairs.next().unwrap())?;
        let identifier = Node::build(pairs.next().unwrap())?;
        let mut datatype = None;
        let mut value = None;

        for pair in pairs {
            match pair.as_rule() {
                Rule::datatype => {
                    datatype = Some(Node::build(pair)?);
                }
                Rule::expression => {
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

impl NodeExecutor for Declaration {
    fn eval(&self, env: &mut ProcessEnv) -> AlthreadResult<Option<Literal>> {
        let datatype = self
            .datatype
            .as_ref()
            .map(|datatype| datatype.value.clone());

        let value = self
            .value
            .as_ref()
            .map(|value| value.eval(env))
            .transpose()?
            .unwrap();

        let mut symbol_table = env.symbol_table.borrow_mut();

        symbol_table
            .insert(
                self.keyword.value == DeclarationKeyword::Let,
                &self.identifier.value,
                datatype,
                value,
            )
            .map_err(|e| {
                AlthreadError::new(
                    ErrorType::VariableError,
                    self.identifier.line,
                    self.identifier.column,
                    e,
                )
            })?;

        Ok(Some(Literal::Null))
    }
}

impl AstDisplay for Declaration {
    fn ast_fmt(&self, f: &mut fmt::Formatter, prefix: &Prefix) -> fmt::Result {
        writeln!(f, "{prefix}decl")?;

        let prefix = &prefix.add_branch();
        writeln!(f, "{prefix}keyword: {}", self.keyword)?;

        match (&self.datatype, &self.value) {
            (Some(datatype), Some(value)) => {
                writeln!(f, "{prefix}ident: {}", self.identifier)?;
                writeln!(f, "{prefix}datatype: {datatype}")?;
                let prefix = prefix.switch();
                writeln!(f, "{prefix}value")?;
                value.ast_fmt(f, &prefix.add_leaf())?;
            }
            (Some(datatype), None) => {
                writeln!(f, "{prefix}ident: {}", self.identifier)?;
                let prefix = prefix.switch();
                writeln!(f, "{prefix}datatype: {datatype}")?;
            }
            (None, Some(value)) => {
                writeln!(f, "{prefix}ident: {}", self.identifier)?;
                let prefix = prefix.switch();
                writeln!(f, "{prefix}value")?;
                value.ast_fmt(f, &prefix.add_leaf())?;
            }
            (None, None) => {
                let prefix = prefix.switch();
                writeln!(f, "{prefix}ident: {}", self.identifier)?;
            }
        }

        Ok(())
    }
}

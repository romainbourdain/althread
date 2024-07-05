use lazy_static::lazy_static;
use pest::{
    iterators::{Pair, Pairs},
    pratt_parser::PrattParser,
};

use crate::{
    env::symbol_table::{DataType, Symbol, SymbolTable},
    error::AlthreadError,
    parser::Rule,
};

pub struct Ast {
    pub nodes: Vec<AstNode>,
}

pub enum AstNode {
    Expr(Expr),
    Decl {
        identifier: String,
        value: Expr,
        datatype: DataType,
        mutable: bool,
    },
}

pub fn parse_prog(pairs: Pairs<Rule>, symbol_table: &mut SymbolTable) -> Result<(), AlthreadError> {
    for pair in pairs {
        match pair.as_rule() {
            Rule::main_block => {
                // TODO : create main block env
                parse_block(pair.into_inner(), symbol_table)?;
            }
            Rule::shared_block => {
                // TODO : implement shared block
                unimplemented!();
            }
            Rule::always_block => {
                // TODO : implement always block
                unimplemented!();
            }
            Rule::EOI => break,
            rule => unreachable!("{:?}", rule),
        }
    }

    Ok(())
}

pub fn parse_block(
    pairs: Pairs<Rule>,
    symbol_table: &mut SymbolTable,
) -> Result<(), AlthreadError> {
    for pair in pairs {
        match pair.as_rule() {
            Rule::decl => {
                parse_declaration(pair.into_inner(), symbol_table)?;
            }
            Rule::expr => {
                // TODO : evaluate expression type
            }
            rule => unreachable!("{:?}", rule),
        }
    }
    Ok(())
}

pub fn parse_declaration(
    pairs: Pairs<Rule>,
    symbol_table: &mut SymbolTable,
) -> Result<(), AlthreadError> {
    let mut symbol = Symbol::new();
    let mut identifier = String::new();
    for pair in pairs {
        match pair.as_rule() {
            Rule::decl_keyword => symbol.mutable = pair.as_str() == "let",

            Rule::IDENTIFIER => {
                identifier = pair.as_str().to_string();
                if symbol_table.contains_key(identifier.as_str()) {
                    return Err(AlthreadError::error(
                        pair.line_col().0,
                        pair.line_col().1,
                        "Variable already declared".to_string(),
                    ));
                }
            }
            Rule::DATATYPE => {
                symbol.datatype = match pair.as_str() {
                    "int" => DataType::Int,
                    "float" => DataType::Float,
                    "bool" => DataType::Bool,
                    "string" => DataType::String,
                    _ => unreachable!(),
                }
            }

            Rule::expr => {
                parse_expr(pair.into_inner())?;
            }
            rule => unreachable!("{:?}", rule),
        }
    }
    symbol_table.insert(identifier, symbol);
    Ok(())
}

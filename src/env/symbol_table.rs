use std::collections::HashMap;

use pest::iterators::Pair;

use crate::{
    error::{AlthreadError, AlthreadResult, ErrorType},
    parser::Rule,
};

use super::{datatype::DataType, value::Value, Environment, Symbol};

pub type SingleSymbolTable = HashMap<String, Symbol>;

#[derive(Debug, Clone)]
pub struct SymbolTable {
    pub symbols: Vec<SingleSymbolTable>,
}

impl SymbolTable {
    pub fn new() -> Self {
        Self {
            symbols: Vec::new(),
        }
    }

    pub fn push(&mut self) {
        self.symbols.push(SingleSymbolTable::new());
    }

    pub fn pop(&mut self) {
        self.symbols.pop();
    }

    pub fn insert(
        &mut self,
        env: &mut Environment,
        mutable: bool,
        identifier: &Pair<Rule>,
        datatype: Option<DataType>,
        value: Option<Value>,
    ) -> AlthreadResult<()> {
        let (line, column) = identifier.line_col();
        let identifier = identifier.as_str().to_string();
        let current_symbol_table = self.symbols.last_mut().unwrap_or(&mut env.global_table);

        if current_symbol_table.contains_key(&identifier) {
            return Err(AlthreadError::new(
                ErrorType::VariableError,
                line,
                column,
                format!("Symbol {} already exists in current scope", identifier),
            ));
        }

        let symbol = Symbol::new(mutable, datatype, value)
            .map_err(|e| AlthreadError::new(ErrorType::VariableError, line, column, e))?;

        current_symbol_table.insert(identifier, symbol);

        Ok(())
    }

    pub fn get<'a>(
        &'a self,
        env: &'a Environment,
        identifier: &Pair<Rule>,
    ) -> AlthreadResult<&'a Symbol> {
        for table in self.symbols.iter().rev() {
            if let Some(symbol) = table.get(identifier.as_str()) {
                return Ok(symbol);
            }
        }

        if let Some(symbol) = env.global_table.get(identifier.as_str()) {
            return Ok(symbol);
        }

        Err(AlthreadError::new(
            ErrorType::VariableError,
            0,
            0,
            format!("Symbol {} not found", identifier.as_str()),
        ))
    }

    pub fn update(
        &mut self,
        env: &mut Environment,
        identifier: &Pair<Rule>,
        value: Value,
    ) -> AlthreadResult<()> {
        for table in self.symbols.iter_mut().rev() {
            if let Some(symbol) = table.get_mut(identifier.as_str()) {
                symbol.update(value).map_err(|e| {
                    AlthreadError::new(
                        ErrorType::VariableError,
                        identifier.line_col().0,
                        identifier.line_col().1,
                        e,
                    )
                })?;
                return Ok(());
            }
        }

        if let Some(symbol) = env.global_table.get_mut(identifier.as_str()) {
            symbol.update(value).map_err(|e| {
                AlthreadError::new(
                    ErrorType::VariableError,
                    identifier.line_col().0,
                    identifier.line_col().1,
                    e,
                )
            })?;
            return Ok(());
        }

        Err(AlthreadError::new(
            ErrorType::VariableError,
            0,
            0,
            format!("Symbol {} not found", identifier.as_str()),
        ))
    }
}

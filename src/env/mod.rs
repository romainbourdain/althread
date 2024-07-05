use symbol_table::{Symbol, SymbolTable};

use crate::{
    ast::{datatype::DataType, expr::PrimaryExpr},
    error::AlthreadError,
};

pub mod symbol_table;

#[derive(Debug)]
pub struct Environment {
    pub symbol_tables: Vec<SymbolTable>,
}

impl Environment {
    pub fn new() -> Self {
        Self {
            symbol_tables: Vec::new(),
        }
    }

    pub fn push_table(&mut self) {
        println!("push scope");
        self.symbol_tables.push(SymbolTable::new());
    }

    pub fn pop_table(&mut self) {
        println!("pop scope: {:?}", self.symbol_tables.pop());
    }

    pub fn insert_symbol(
        &mut self,
        identifier: String,
        datatype: DataType,
        mutable: bool,
        value: Option<PrimaryExpr>,
    ) -> Result<(), AlthreadError> {
        let current_symbol_table: &mut std::collections::HashMap<String, Symbol> = self
            .symbol_tables
            .last_mut()
            .ok_or_else(|| AlthreadError::error(0, 0, "No symbol table found".to_string()))?;

        if current_symbol_table.contains_key(&identifier) {
            return Err(AlthreadError::error(
                0,
                0,
                format!("Symbol {} already exists in current scope", identifier),
            ));
        }

        let symbol = Symbol {
            datatype,
            mutable,
            value,
        };

        println!("inserting symbol: {}", identifier);

        current_symbol_table.insert(identifier, symbol);
        Ok(())
    }

    pub fn get_symbol(&self, identifier: &String) -> Result<&Symbol, AlthreadError> {
        for table in self.symbol_tables.iter().rev() {
            if let Some(symbol) = table.get(identifier) {
                return Ok(symbol);
            }
        }
        Err(AlthreadError::error(
            0,
            0,
            format!("Symbol {} not found", identifier),
        ))
    }
}

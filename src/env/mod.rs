use symbol_table::{Symbol, SymbolTable};

use crate::{
    ast::{datatype::DataType, expr::PrimaryExpr},
    error::{AlthreadError, ErrorType},
};

pub mod symbol_table;

#[derive(Debug)]
pub struct Environment<'a> {
    pub symbol_tables: Vec<SymbolTable>,
    pub global_table: &'a mut SymbolTable,
}

impl<'a> Environment<'a> {
    pub fn new(global_table: &'a mut SymbolTable) -> Self {
        Self {
            symbol_tables: Vec::new(),
            global_table,
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
        let current_symbol_table = self
            .symbol_tables
            .last_mut()
            .unwrap_or(&mut self.global_table);

        if current_symbol_table.contains_key(&identifier) {
            return Err(AlthreadError::error(
                ErrorType::VariableError,
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

    pub fn get_symbol(&self, identifier: &String) -> Result<&Symbol, String> {
        for table in self.symbol_tables.iter().rev() {
            if let Some(symbol) = table.get(identifier) {
                return Ok(symbol);
            }
        }

        if let Some(symbol) = self.global_table.get(identifier) {
            return Ok(symbol);
        }

        Err(format!("Symbol {} not found", identifier))
    }
}
